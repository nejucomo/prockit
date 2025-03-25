//! A simple framework for defining proc macros.
//!
//! The primary traits for proc macro implementations are the [ProcMacroDerive], (TODO) `ProcMacro`, and (TODO) `ProcMacroAttribute`.
#![deny(missing_docs)]

mod deriveimpls;
mod diwithattrs;
pub mod ext;
mod fromattr;

pub use proc_macro2;
pub use quote;
pub use syn;

pub use crate::deriveimpls::ProcMacroDeriveBase;
pub use crate::diwithattrs::{PmdInput, ProcMacroDerive};
pub use crate::fromattr::FromAttr;
