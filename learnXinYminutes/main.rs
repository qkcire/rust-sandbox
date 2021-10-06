// This is a comment. Line comments look
// like this...
// and extend multiple lines like this.

/// Documentation comments look like this and
/// support markdown notation.
/// # Examples
///
/// ```
/// let five = 5
/// ```

///////////////
// 1. Basics //
///////////////

#[allow(dead_code)]
// Functions
// `i32` is the type for 32-bit signed integers
fn add2(x: i32, y: i32) -> i32 {
  // Implicit return (no semicolon)
  x + y
}