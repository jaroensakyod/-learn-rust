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
fn main() {
          let mut n1 = 1;
          let n2 = &mut n1;
          println!("")

}