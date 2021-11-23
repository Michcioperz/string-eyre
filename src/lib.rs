/// This crate provides convenience traits for pushing into `eyre`
/// errors that shouldn't go into `eyre` by stringifying them.

use std::fmt::{Debug, Display};

use eyre::{eyre, Report};

pub trait DisplayToEyre<T, E: Display>: private::Sealed {
    /// Convert the error value to `eyre::Report` of its `Display` representation.
    fn display_to_eyre(self) -> Result<T, Report>;
}

pub trait DebugToEyre<T, E: Debug>: private::Sealed {
    /// Convert the error value to `eyre::Report` of its `Debug` representation.
    fn debug_to_eyre(self) -> Result<T, Report>;
}

impl<T, E: Display> DisplayToEyre<T, E> for Result<T, E> {
    #[inline]
    fn display_to_eyre(self) -> Result<T, Report> {
        self.map_err(|e| eyre!("{}", e))
    }
}

impl<T, E: Debug> DebugToEyre<T, E> for Result<T, E> {
    #[inline]
    fn debug_to_eyre(self) -> Result<T, Report> {
        self.map_err(|e| eyre!("{:?}", e))
    }
}

mod private {
    pub trait Sealed {}
    impl<T, E> Sealed for Result<T, E> {}
}
