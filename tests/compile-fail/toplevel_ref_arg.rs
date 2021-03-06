#![feature(plugin)]

#![plugin(clippy)]
#![deny(clippy)]
#![allow(unused)]

fn the_answer(ref mut x: u8) {  //~ ERROR `ref` directly on a function argument is ignored
  *x = 42;
}

fn main() {
  let mut x = 0;
  the_answer(x);
  // Closures should not warn
  let y = |ref x| { println!("{:?}", x) };
  y(1u8);

  let ref x = 1;
  //~^ ERROR `ref` on an entire `let` pattern is discouraged
  //~| HELP try
  //~| SUGGESTION let x = &1;

  let ref y: (&_, u8) = (&1, 2);
  //~^ ERROR `ref` on an entire `let` pattern is discouraged
  //~| HELP try
  //~| SUGGESTION let y: (&_, u8) = &(&1, 2);

  let ref z = 1 + 2;
  //~^ ERROR `ref` on an entire `let` pattern is discouraged
  //~| HELP try
  //~| SUGGESTION let z = &(1 + 2);

  let (ref x, _) = (1,2); // okay, not top level
  println!("The answer is {}.", x);
}
