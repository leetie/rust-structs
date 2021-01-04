pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}

pub fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
// mod area {
//   pub fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
//   }
//   pub struct Rectangle {
//     width: u32,
//     height: u32,
//   }
// }
