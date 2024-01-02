//
// Rust program execution starts
// from standard-library function
// main()
//
fn main() -> () {
    let hello_world : &str = "Hello, World!";
    println!(
      "{}",
      hello_world
    );
  //
  // Return an empty tuple also
  // known as an unit to the
  // underlying operating-system
  //
  return;
}