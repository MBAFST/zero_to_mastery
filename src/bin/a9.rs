// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5

// * Use a function that returns a tuple
fn coordinate() -> (i32, i32) {
  (1, 2)
}

fn main() {
// * Destructure the return value into two variables
  let (_, y) = coordinate();
// * Use an if..else if..else block to determine what to print
  if y > 5 {
    println!(">5");
  }
  else if y < 5 {
    println!("<5");
  }
  else {
    println!("=5");
  }
}
