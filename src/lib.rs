//! This crate provides a Hardware Abstraction Layer for Power Sequencing.

#![doc(html_root_url = "https://docs.rs/embedded-keyboard/latest")]
#![cfg_attr(not(test), no_std)]
#![allow(async_fn_in_trait)]

/// Regulator error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Regulator error kind.
    ///
    /// By using this method, Regulator errors freely defined by HAL
    /// implementations can be converted to a set of generic Regulator
    /// errors upon which generic code can act.
    fn kind(&self) -> ErrorKind;
}

/// Regulator error kind.
///
/// This represents a common set of errors. HAL implementations are free to
/// define more specific or additional error types. However, by providing a
/// mapping to these common Regulator errors, generic code can still react
/// to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// A different error occurred. The original error may contain more
    /// information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Regulator error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

pub trait Regulator: ErrorType {
    async fn enable(&mut self) -> Result<(), Self::Error>;
    async fn disable(&mut self) -> Result<(), Self::Error>;
}

impl<T: Regulator + ?Sized> Regulator for &mut T {
    #[inline]
    async fn enable(&mut self) -> Result<(), Self::Error> {
        T::enable(self).await
    }

    #[inline]
    async fn disable(&mut self) -> Result<(), Self::Error> {
        T::disable(self).await
    }
}
