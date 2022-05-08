//Repeating Code with loop
// fn main() {
//           let mut count = 0;
//           'counting_up: loop {
//               println!("count = {}", count);
//               let mut remaining = 10;
      
//               loop {
//                   println!("remaining = {}", remaining);
//                   if remaining == 4 {
//                       break;
//                   }
//                   if count == 4 {
//                       break 'counting_up;
//                   }
//                   remaining -= 1;
//               }
      
//               count += 1;
//           }
//           println!("End count = {}", count);
//       }
      
//Returning Values from Loops
// fn main() {
//           let mut counter = 0;
      
//           let result = loop {
//               counter += 1;
//               println!("{}",counter);
      
//               if counter == 10 {
//                   break counter * 2;
//               }
//           };
      
//           println!("The result is {}", result);
//       }
      
// Conditional Loops with while
// fn main() {
//           let mut number = 8;
      
//           while number != 0 {
//               println!("{}!", number);
      
//               number -= 1;
//           }
      
//           println!("LIFTOFF!!!");
//       }

// Looping Through a Collection with for
// fn main() {
//           let a = [10, 20, 30, 40, 50];
//           let mut index = 0;
      
//           while index < 5 {
//               println!("the value is: {}", a[index]);
      
//               index += 1;
//           }
//       }
// fn main() {
//           let a = [10, 20, 30, 40, 50];
      
//           for element in a {
//               println!("the value is: {}", element);
//           }
//       }
      
      
// fn main() {
//           for number in (1..=4).rev() {
//               println!("{}!", number);
//           }
//           println!("LIFTOFF!!!");
//       }
// fn first_word(s: &String) -> usize {
//           let bytes = s.as_bytes();
      
//           for (i, &item) in bytes.iter().enumerate() {
//               if item == b' ' {
//                   return i;
//               }
//           }
      
//           s.len()
//       }
      
//       fn main() {}


fn main() {
          let s = String::from("abc");
          hello(&s1);
          println!("{}", s1);

}
fn hello(name: &String) {
          println!("{}", name);
}