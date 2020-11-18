/// Trait for equality comparisons that are [approximately equal](https://en.wikipedia.org/wiki/Approximation)
///
///
/// This trait allows for approximate equality, for results that just have to be *"good enough"*.
///
/// Herein `a ~= b` implies that `a.aeq(b)` and `a !~=` implies `a.nae(b)`.
///
/// The approximate equality, however, must be (for all `a`, `b` and `c`)
/// symmetric, in that: `a ~= b` implies `b ~= a`.
///
/// ## How can I implement `ApproxEq`?
///
/// `ApproxEq` only requires the [`aeq`] method be implemented; [`nae`] is defined
/// in terms of it by default. Any implementation of [`ane`] *must* respect
/// the rule that [`aeq`] is a strict inverse of [`ane`]; that is, `!(a ~= b)` if
/// and only if `a !~= b`.
///
/// An example implementation for a domain in which two books are considered
/// the same book if their ISBNs have the same parity, even if the formats differ:
///
/// ```
/// use approxeq::ApproxEq;
///
/// enum BookFormat {
///     Paperback,
///     Hardback,
///     Ebook,
/// }
///
/// struct Book {
///     isbn: i32,
///     format: BookFormat,
/// }
///
/// impl ApproxEq for Book {
///     fn aeq(&self, other: &Self) -> bool {
///       self.isbn % 2 == other.isbn % 2
///     }
/// }
///
/// let b1 = Book { isbn: 3, format: BookFormat::Paperback };
/// let b2 = Book { isbn: 3, format: BookFormat::Ebook };
/// let b3 = Book { isbn: 10, format: BookFormat::Paperback };
///
/// assert!(b1.aeq(&b2));
/// assert!(b1.nae(&b3));
/// ```
/// 
/// ## How can I compare two different types?
///
/// The type you can compare with is controlled by `ApproxEq`'s type parameter.
/// For example, let's tweak our previous code a bit:
///
/// ```
/// use approxeq::ApproxEq;
///
/// #[derive(PartialEq)]
/// enum BookFormat {
///     Paperback,
///     Hardback,
///     Ebook,
/// }
///
/// struct Book {
///     isbn: i32,
///     format: BookFormat,
/// }
///
/// // Implement <Book> ~= <BookFormat> comparisons
/// impl ApproxEq<BookFormat> for Book {
///     fn aeq(&self, other: &BookFormat) -> bool {
///         match self.format {
///             BookFormat::Ebook => self.format == *other,
///             _ => other != &BookFormat::Ebook,
///         }
///     }
/// }
///
/// // Implement <BookFormat> ~= <Book> comparisons
/// impl ApproxEq<Book> for BookFormat {
///     fn aeq(&self, other: &Book) -> bool {
///         *self == BookFormat::Ebook && other.format == BookFormat::Ebook ||
///             *self != BookFormat::Ebook && other.format != BookFormat::Ebook
///     }
/// }
///
/// let b1 = Book { isbn: 3, format: BookFormat::Paperback };
///
/// assert!(b1.aeq(&BookFormat::Paperback));
/// assert!(BookFormat::Ebook.nae(&b1));
/// ```
///
/// By changing `impl ApproxEq for Book` to `impl ApproxEq<BookFormat> for Book`,
/// we allow `BookFormat`s to be compared with `Book`s.
pub trait ApproxEq<Rhs: ?Sized = Self> {
    fn aeq(&self, other: &Rhs) -> bool;

    fn nae(&self, other: &Rhs) -> bool {
        !self.aeq(other)
    }
}
