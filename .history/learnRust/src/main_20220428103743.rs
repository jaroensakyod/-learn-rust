#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(while_true)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]


use learnRust::person::P;
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

           // let result = score >= 50 ? "Pass" : "Fail";
    let result = if score >= 50 { "Pass" } else { "Fail" };

    //Loop
    while true {
        break;
    }

    'label1: loop {
        'label2: loop {
            break 'label1;
            continue 'label2;
        }
    }

    for i in 0..3 {
        println!("{}", i);
    }

    for i in 0..=3 {
        println!("{}", i);
    }

    let numbers = [10, 20, 30];
    for n in numbers.iter() {
        println!("{}", n);
    }
    for n in [10, 20, 30].iter() {
        println!("{}", n);
    }

    let numbers = [(1, 2), (3, 4)];
    for (i, j) in numbers.iter() {
        println!("{} {}", i, j);
    }

    //String
    let x = "Hello";
    let x = String::from("Hello");
    let x = "Hello".to_string();

    //Collection
    let mut x: Vec<i32> = Vec::new();
    x.push(10);
    x.push(20);
    x.push(30);
    let y = x.pop();
    let mut x = vec![1, 2, 3];

    //HashMap
    let mut x: HashMap<&str, &str> = HashMap::new();
    x.insert("th", "Thailand");
    x.insert("us", "United State");
    let y = x.get("th");
    println!("{}", y.unwrap());
}
