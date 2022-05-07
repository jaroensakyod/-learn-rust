#![allow(unused_variables)]
//debug
// fn main() {
//     let n1 = 1;
//     let n2 = 2;
//     ex1_1();
//     println!();
//     println!();
// }
// fn ex1_1() {
//           let n3 = 3;
//           ex1_2();
//           println!();
//           println!();
// }
// fn ex1_2() {
//           let n4 = 4;
//           println!();
// }

// borrowing n2 changes n1
// fn main() {
//           let mut n1 = 1;
//           let n2 = &mut n1; //n2 borrows n1
//           *n2 = 20;
//           println!("{}", n1);
          

// }

// mutable borrows function
// fn main() {
//           let mut n1 = 1;
//           hello(&mut n1);
//           println!("{}",n1);


// }
// fn hello(a:&mut i32) {
//           *a = 30;
          
// }
fn main() {
          let mut s = String::from("hello");
      
          change(&mut s);
          println!("{}", s);
      }
      
      fn change(some_string: &mut String) {
          some_string.push_str(", world");
      }
