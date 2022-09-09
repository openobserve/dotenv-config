/* Copyright 2022 Zinc Labs Inc. and Contributors
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

//! dotenv_config provides a way to read configuration from environment variables.
//!
//!
//! use `.env` as config file and parse environments to config struct.
//!
//! ## Usage
//!
//! ### derive EnvConfig
//!
//! ```rust
//! use dotenv_config::EnvConfig;
//!
//! #[derive(Debug, EnvConfig)]
//! struct Config {
//!     #[env_config(default = "192.168.2.1")]
//!     server_addr: String,
//!     server_mode: bool,
//!     #[env_config(name = "ZINC_FOO", default = true)]
//!     foo: bool,
//!     #[env_config(name = "ZINC_BAR", default = 123456)]
//!     bar: Option<i64>,
//! }
//!
//! fn main() {
//!     let cfg = Config::init().unwrap();
//!     println!("{:#?}", cfg);
//! }
//! ```
//!
//! ### attribute env_config
//!
//! you can use macro attribute set field attribute
//!
//! - name: change default environment key
//! - default: if not set, used as default value
//!
//! ## you can though system environments or `.env` file config it.
//!
//! ```ignore
//! ZINC_FOO=false
//! ZINC_BAR=8787878
//! ```
//!
//! default load environment key is: `structName_fieldName` do UpperSnake, like above struct, default config key is:
//!
//! ```ignore
//! CONFIG_SERVER_ADDR
//! CONFIG_SERVER_MODE
//! ZINC_FOO
//! ZINC_BAR
//! ```
//!
//! If you have some problems please go to github create a issue.
//! https://github.com/zinclabs/dotenv-config
//!
extern crate proc_macro;

use proc_macro::TokenStream;
mod builder;

#[proc_macro_derive(EnvConfig, attributes(env_config))]
pub fn derive_env_config(input: TokenStream) -> TokenStream {
    // println!("{:#?}", input);
    builder::BuilderContext::render(input)
        .unwrap()
        .parse()
        .unwrap()
}
