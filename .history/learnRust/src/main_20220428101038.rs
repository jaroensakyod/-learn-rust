#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(while_true)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
    println!("Hello, world!");
    let x: i32 =10;
    println!("{}", x);

    
fn main() {
          //Variables
          let mut x: i32;
          x = 10;
          x = 20;
      
          let x = 10;
          let (x, y) = (10, 20);
      
          const PI: f64 = 3.14;
      
          //Tuple
          let x = (1, 3.14, 1000);
          let x: (u8, f64, i32) = (1, 3.14, 1000);
          let a = x.0;
          let b = x.1;
          let c = x.2;
      
          //Array
          let x: [i32; 5];
          let x = [1, 2, 3, 4, 5];
          let x = [0; 5];
      
          //If
          let score = 50;
      
          let mut grade = "";
          if score >= 80 {
              grade = "A";
          } else if score >= 70 {
              grade = "B";
          } else if score >= 60 {
              grade = "C";
          } else if score >= 50 {
              grade = "D";
          } else {
              grade = "F";
          }
      
          let grade = if score >= 80 {
              "A"
          } else if score >= 70 {
              "B"
          } else if score >= 60 {
              "C"
          } else if score >= 50 {
              "D"
          } else {
              "F"
          };
      
}
