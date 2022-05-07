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
      
fn main() {
          let mut counter = 0;
      
          let result = loop {
              counter += 1;
      
              if counter == 10 {
                  break counter * 2;
              }
          };
      
          println!("The result is {}", result);
      }
      