/// Extension trait that allows values to be wrapped into [`Option`] using method syntax.
///
/// # Examples
///
/// ```rust
/// use implicitly::IntoOption;
///
/// assert_eq!(42.some(), Some(42));
/// assert_eq!(i32::none(), None::<i32>);
/// ```
pub trait IntoOption: Sized {
    /// Wraps `self` in [`Some`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use implicitly::IntoOption;
    ///
    /// assert_eq!("hello".some(), Some("hello"));
    /// ```
    fn some(self) -> Option<Self>;

    /// Returns [`None`] for this type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use implicitly::IntoOption;
    ///
    /// assert_eq!(i32::none(), None::<i32>);
    /// ```
    fn none() -> Option<Self>;
}

impl<T: Sized> IntoOption for T {
    fn some(self) -> Option<Self> {
        Some(self)
    }

    fn none() -> Option<Self> {
        None
    }
}

/// Extension trait that allows values to be wrapped into [`Result`] using method syntax.
///
/// # Examples
///
/// ```rust
/// use implicitly::IntoResult;
///
/// assert_eq!(42.ok::<&str>(), Ok(42));
/// assert_eq!("error".err::<i32>(), Err("error"));
/// ```
pub trait IntoResult: Sized {
    /// Wraps `self` in [`Ok`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use implicitly::IntoResult;
    ///
    /// assert_eq!(42.ok::<&str>(), Ok(42));
    /// ```
    fn ok<E>(self) -> Result<Self, E>;

    /// Wraps `self` in [`Err`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use implicitly::IntoResult;
    ///
    /// assert_eq!("error".err::<i32>(), Err("error"));
    /// ```
    fn err<A>(self) -> Result<A, Self>;
}

impl<T: Sized> IntoResult for T {
    fn ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }

    fn err<A>(self) -> Result<A, Self> {
        Err(self)
    }
}
