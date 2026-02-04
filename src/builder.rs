/* Copyright 2026 OpenObserve Inc. and Contributors
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
 */

use anyhow::Result;
use askama::Template;
use convert_case::{Case, Casing};
use proc_macro::{Ident, TokenStream, TokenTree};
use std::collections::VecDeque;

#[derive(Template)]
#[template(path = "builder.j2", escape = "none")]
pub struct BuilderContext {
    name: String,
    fields: Vec<Fd>,
}

impl BuilderContext {
    pub fn contains(&self, haystack: &[&str], needle: &str) -> bool {
        haystack.contains(&needle)
    }

    pub fn uppersnake(&self, s: &str) -> String {
        s.to_case(Case::UpperSnake)
    }
}

#[derive(Debug, Default)]
struct Fd {
    name: String,
    typ: String,
    optional: bool,
    attr_name: String,
    attr_default: String,
    attr_help: String, // new field for storing documentation comments
    attr_parse: bool,  // whether use FromStr trait to parse into a field
}

impl Fd {
    pub fn new(name: &[TokenTree], typ: &[TokenTree]) -> Self {
        // collect Ident("Option"), Punct('<'), Ident("String"), Punct('>') into a String vec
        // like: vec!["Option", "<", "String", ">"]

        // find env_config Group
        let mut attr_name: String = String::from("");
        let mut attr_default: String = String::from("");
        let mut attr_help: String = String::from("");
        let mut attr_parse: bool = false;
        for item in name {
            if let TokenTree::Group(g) = item {
                let mut g = g.stream().into_iter();
                let ident = g.next().unwrap();
                if ident.to_string() == "env_config" {
                    let ident = g.next().unwrap();
                    if let TokenTree::Group(g) = ident {
                        let attrs = get_struct_attribute(g.stream());
                        for item in attrs {
                            match item.0.as_str() {
                                "name" => {
                                    attr_name = item.1;
                                }
                                "default" => {
                                    attr_default = item.1;
                                }
                                "help" => {
                                    attr_help = item.1;
                                }
                                "parse" => {
                                    if item.1.is_empty() {
                                        attr_parse = true;
                                    } else {
                                        attr_parse = item.1.parse().unwrap();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    break;
                }
            }
        }

        let typ = typ
            .iter()
            .map(|v| match v {
                TokenTree::Ident(n) => n.to_string(),
                TokenTree::Punct(p) => p.as_char().to_string(),
                e => panic!("Expect ident, but got {:?}", e),
            })
            .collect::<Vec<_>>();

        // it's name of field that last TokenTree before Punct(':')
        // eg: executable: String,
        // warn: there not use name[0], because it maybe `pub executable: String`
        match name.last() {
            Some(TokenTree::Ident(name)) => {
                // if typ first is Option, then from second take last
                let (typ, optional) = if typ[0].as_str() == "Option" {
                    (&typ[2..typ.len() - 1], true)
                } else {
                    (&typ[..], false)
                };
                Self {
                    name: name.to_string(),
                    typ: typ.join(""),
                    optional,
                    attr_name,
                    attr_default,
                    attr_help,
                    attr_parse,
                }
            }
            e => panic!("Expect ident, but got {:?}", e),
        }
    }
}

impl BuilderContext {
    /// build BuilderContext from TokenStream
    fn new(input: TokenStream) -> Self {
        let (name, input) = split(input);
        let fields = get_struct_fields(input);
        Self {
            name: name.to_string(),
            fields,
        }
    }

    /// render template to code Token
    pub fn render(input: TokenStream) -> Result<String> {
        let template = Self::new(input);
        Ok(template.render()?)
    }
}

/// split TokenStream to struct name, fields
fn split(input: TokenStream) -> (Ident, TokenStream) {
    let mut input = input.into_iter().collect::<VecDeque<_>>();
    while let Some(item) = input.pop_front() {
        if let TokenTree::Ident(v) = item
            && v.to_string() == "struct"
        {
            break;
        }
    }

    // struct name should behind struct
    let ident;
    if let Some(TokenTree::Ident(v)) = input.pop_front() {
        ident = v;
    } else {
        panic!("Didn't find struct name");
    }

    // find first Group
    let mut group = None;
    for item in input {
        if let TokenTree::Group(g) = item {
            group = Some(g);
            break;
        }
    }

    (ident, group.expect("Didn't find field group").stream())
}

/// find all Fd from TokenStream
fn get_struct_fields(input: TokenStream) -> Vec<Fd> {
    let input = input.into_iter().collect::<Vec<_>>();
    input
        .split(|v| match v {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        })
        .map(|tokens| {
            tokens
                .split(|v| match v {
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>()
        })
        .filter(|tokens| tokens.len() == 2)
        .map(|tokens| Fd::new(tokens[0], tokens[1]))
        .collect()
}

/// find all attribute from TokenStream
fn get_struct_attribute(input: TokenStream) -> Vec<(String, String)> {
    let input = input.into_iter().collect::<Vec<_>>();
    input
        .split(|v| match v {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        })
        .map(|tokens| {
            tokens
                .split(|v| match v {
                    TokenTree::Punct(p) => p.as_char() == '=',
                    _ => false,
                })
                .collect::<Vec<_>>()
        })
        .map(|tokens| {
            let token0 = tokens[0]
                .last()
                .unwrap()
                .to_string()
                .trim_matches(|c: char| c == '"' || c == '\'')
                .to_string();
            let token1 = if tokens.len() > 1 {
                tokens[1]
                    .last()
                    .unwrap()
                    .to_string()
                    .trim_matches(|c: char| c == '"' || c == '\'')
                    .to_string()
            } else {
                String::from("")
            };
            (token0, token1)
        })
        .collect()
}
