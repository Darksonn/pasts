#![no_std]
extern crate alloc;

mod waker;

pub use waker::{Woke};

/// Pin a variable to a location in the stack.
///
/// ```rust
/// pasts::let_pin! {
///     var = "Hello, world";
/// };
/// let _: core::pin::Pin<&mut &str> = var;
/// ```
#[macro_export]
macro_rules! let_pin {
    {$($x:ident = $y:expr),* $(;)?} => { $(
        // Force move.
        let mut $x = $y;
        // Shadow to prevent future use.
        #[allow(unused_mut)]
        let mut $x = unsafe { core::pin::Pin::new_unchecked(&mut $x) };
    )* }
}
