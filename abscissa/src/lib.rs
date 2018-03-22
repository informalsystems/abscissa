//! Abscissa: an app microframework
//!
//! Abscissa is a microframework for building Rust applications (either CLI tools
//! or network services), aiming to provide a large number of features with a
//! minimal number of dependencies, and with a strong focus on security.
//!
//! ## Features
//!
//! - **command-line option parsing**: simple declarative option parser based on
//!   (i.e. forked from) [gumdrop]. The option parser in Abcissa contains numerous
//!   improvements which provide better UX and tighter integration with the other
//!   parts of the framework (e.g. overriding configuration settings using
//!   command-line options).
//! - **configuration**: declarative global configuration support using a [RwLock]
//!   on a [lazy_static]. Simple parsing of TOML configurations to serde-parsed
//!   global structures which can be dynamically updated at runtime.
//! - **error handling**: generic `Error` type based on the `failure` crate, and a
//!   unified error-handling subsystem.
//! - **logging**: uses the `log` and `simplelog` crates to automatically configure
//!   application-level logging, presently to standard output or files.
//! - **secrets management**: the (optional) `secrets` module includes a `Secret`
//!  type which derives serde's `Deserialize` and can be used to represent secret
//!  values parsed from configuration files or elsewhere (e.g. credentials loaded
//!  from the environment or network requests)
//! - **shell interactions**: support for colored terminal output (with color
//!   support autodetection). Useful for Cargo-like status messages with
//!   easy-to-use macros.
//!
//! [gumdrop]: https://github.com/murarth/gumdrop
//! [RwLock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
//! [lazy_static]: https://docs.rs/lazy_static
//!
//! # Option Parser
//!
//! Please see the documentation for the `options` module.
//!
//! # Status Macros
//!
//! ```
//! # #[macro_use] extern crate abscissa;
//! # fn main() {
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//!
//! // Print an error message
//! status_err!("something bad happened");
//!
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//!
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! # }
//! ```

#![crate_name = "abscissa"]
#![crate_type = "rlib"]
#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/abscissa/0.0.0")]
#![doc(
    html_logo_url = "https://www.iqlusion.io/img/github/iqlusioninc/crates/abscissa/abscissa-sq.svg",
    html_root_url = "https://docs.rs/abscissa/0.0.0"
)]

#[allow(unknown_lints, unused_imports, useless_attribute)]
#[macro_use]
extern crate abscissa_derive;
pub extern crate failure;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "logging")]
pub extern crate log;
#[cfg(feature = "config")]
extern crate serde;
#[cfg(feature = "simplelog")]
extern crate simplelog;
extern crate term;

#[cfg(all(test, feature = "options"))]
#[macro_use]
extern crate assert_matches;

// Load macros first
#[macro_use]
pub mod macros;

#[cfg(feature = "application")]
mod application;
#[cfg(feature = "options")]
mod command;
#[cfg(feature = "config")]
pub mod config;
mod error;
#[cfg(feature = "logging")]
mod logging;
#[cfg(feature = "options")]
pub mod options;
#[cfg(feature = "secrets")]
pub mod secrets;
pub mod shell;
pub mod util;

#[cfg(feature = "application")]
pub use application::{boot, Application, ApplicationPath, Component, Components};
#[cfg(feature = "options")]
pub use command::{Callable, Command};
pub use config::{ConfigReader, GlobalConfig};
pub use error::{Error, Fail, FrameworkError, FrameworkErrorKind};
#[cfg(feature = "logging")]
pub use logging::LoggingConfig;
#[cfg(feature = "options")]
pub use options::Options;
#[cfg(feature = "secrets")]
pub use secrets::Secret;
pub use shell::{status, ColorConfig, Stream};
#[cfg(feature = "application")]
pub use util::Version;