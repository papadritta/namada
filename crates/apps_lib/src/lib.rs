//! Shared code for the Namada apps.

#![doc(html_favicon_url = "https://dev.namada.net/master/favicon.png")]
#![doc(html_logo_url = "https://dev.namada.net/master/rustdoc-logo.png")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![warn(
    rust_2018_idioms,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::arithmetic_side_effects,
    clippy::dbg_macro
)]

pub mod cli;
pub mod client;
pub mod config;
pub mod logging;
pub mod tendermint_node;
pub mod wallet;
pub mod wasm_loader;

// This is here only to include the std's docs in our docs.
// Taken from <https://github.com/rust-lang/rfcs/issues/2324#issuecomment-502437904>.
#[doc(inline)]
pub use std;

pub mod facade {
    // TODO(namada#3248): only re-export v037 `tendermint-rs`
    pub use namada::{tendermint, tendermint_proto, tendermint_rpc};
    pub use tendermint_config;
}
