//! A simple framework for defining proc macros.
//!
//! The primary traits for proc macro implementations are the [ProcMacroDerive], `ProcMacro` (TODO), and `ProcMacroAttribute` (TODO).
#![deny(missing_docs)]

pub mod ext;
mod pmderive;

pub use proc_macro2;
pub use quote;
pub use syn;

pub use crate::pmderive::{DeriveInput, ProcMacroDerive};
