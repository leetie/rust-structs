#[derive(Debug)]
pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}
impl Rectangle {
  pub fn area(&self) -> u32 {
    // reference as parameter here as to not take ownership of...self 😐
    self.width * self.height
  }
}
// pub fn area(rectangle: &Rectangle) -> u32 {
//   rectangle.width * rectangle.height
// }
// mod area {
//   pub fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
//   }
//   pub struct Rectangle {
//     width: u32,
//     height: u32,
//   }
// }
