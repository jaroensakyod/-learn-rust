// fn main() {
//           let width1 = 30;
//           let height1 = 50;
      
//           println!(
//               "The area of the rectangle is {} square pixels.",
//               area(width1, height1)
//           );
//       }
      
//       fn area(width: u32, height: u32) -> u32 {
//           width * height
//       }

//Refactoring with Tuples
// fn main() {
//           let rect1 = (30, 50);
      
//           println!(
//               "The area of the rectangle is {} square pixels.",
//               area(rect1)
//           );
//       }
      
//       fn area(dimensions: (u32, u32)) -> u32 {
//           dimensions.0 * dimensions.1
//       }

//Refactoring with Structs: Adding More Meaning

// struct Rectangle {
//           width: u32,
//           height: u32,
//       }
      
//       fn main() {
//           let rect1 = Rectangle {
//               width: 30,
//               height: 50,
//           };
      
//           println!(
//               "The area of the rectangle is {} square pixels.",
//               area(&rect1)
//           );
//       }
      
//       fn area(rectangle: &Rectangle) -> u32 {
//           rectangle.width * rectangle.height
//       }
// #[derive(Debug)]

// struct Rectangle {
//           width: u32,
//           height: u32,
//       }
      
//       fn main() {
//           let rect1 = Rectangle {
//               width: 30,
//               height: 50,
//           };
      
//           println!("rect1 is {:#?}", rect1);
//       }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

//Defining Methods
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }

//Methods with More Parameters

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }
