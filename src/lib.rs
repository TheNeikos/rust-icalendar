//! # iCalendar
//! iCalendar is a format specified by the
//! [RFC5545](https://www.ietf.org/rfc/rfc5545.txt). This library allows you
//! to parse and create valid iCalendar strings.

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]


/// The component itself
pub mod component;

pub use component::Component;
