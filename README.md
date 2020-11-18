# ApproxEq

[![Crate Documentation](https://img.shields.io/badge/docs.rs-approxeq-green.svg)](https://docs.rs/approxeq)
[![Current Crates.io Version](https://img.shields.io/crates/v/approxeq.svg)](https://crates.io/crates/approxeq)

The `ApproxEq` trait handily provides a way to define approximate relations between types and comes with already-declared arbitrary implementations for primitive number types!

Easily define implementations for your own types:
```rust
use approxeq::ApproxEq;

enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl ApproxEq for Book {
    // Two books are approximately equal when their respective ISBNs parity matches
    fn aeq(&self, &other: Self) -> bool {
        self.isbn % 2 == other.isbn %2
    }
}

fn main() {
    let b1 = Book { isbn: 3, format: Paperback }
    let b2 = Book { isbn: 5, format: Hardback }
    let b3 = Book { isbn: 10, format: Ebook }

    assert!(b1.aeq(&b2));
    assert!(b1.nae(&b3));
}
```

## Contributing
Any and all PRs are welcome.

## Acknowledgement
The ApproxEq crate is what you might call a joke.

Much of the documentation is an adaptation of rust-lang's `std::cmp::PartialEq` trait documentation.
