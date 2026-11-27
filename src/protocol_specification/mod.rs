//! A [`Template`] describes the the overall shape of a protocol. That is,
//! the message formats, field semantics, message sequence, use of crypto,
//! and extra options. It does not define a protocol yet, because values are
//! not defined: in particular, the value of fixed fields and passwords are
//! note defined in the template.
//!
//! [`Assignments`] is what is used to assign values to fields; it is just a
//! mapping of field names to values. Note that fields with the random semantic
//! do not get assignments because those are sampled at protocol runtime.
//! Assignments are used to give values to fixed fields at compile time.
//!
//! A [`ProtocolSpecification`] instance is just a coupling of a template with
//! value assignments. To get an instance, [`binder::bind`] is the function
//! to use. [`binder::bind`] performs the validity checks to ensure that the
//! bindings are good for the template.
//!
//! [`ProtocolSpecification`] implements [`Display`](std::fmt::Display)
//! which can be used to serialize a protocol. When written to a file,
//! we call this a protocol specification file, or psf.
//!
//! The workflow is shown here:
//!
//! ```text
//!    ┌───────────┐
//!    │ Template  │──┐
//!    │           │  └──▶┌─────────┐                ┌───────────┐                  ┌──────┐
//!    └───────────┘      │ Binder  │──────bind─────▶│ Instance  │─────to_string───▶│ psf  │
//! ┌──────────────┐  ┌──▶│         │                │           │                  │      │
//! │ Assignments  │──┘   └─────────┘                └───────────┘                  └──────┘
//! │              │
//! └──────────────┘
//!
//! ```

mod assignments;
pub mod binder;
mod display;
mod template;

pub use assignments::{Assignments, HexLiteral, LiteralValue};
pub use template::*;

/// A protocol instance produced by binding [`Assignments`] to a [`Template`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolSpecification {
    template: Template,
    assignments: Assignments,
}

impl ProtocolSpecification {
    pub fn template(&self) -> &Template {
        &self.template
    }

    pub fn assignments(&self) -> &Assignments {
        &self.assignments
    }
}
