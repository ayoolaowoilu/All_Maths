use crate::equations::{bodmas::calculate_bodmas, pemdas::calculate_pemdas};
use crate::utils::element_seperator::map_elements;

mod equations;
mod utils;

fn main() {
    println!("Hello, world!");
   let el =  map_elements("-5x + 4*67 + 90 - 90+9/2 = 10");

  for e in el.left_side.iter() {
    println!("Left side element: {} of type {}", e.value, e.element_type);
  }
  for e in el.right_side.as_ref().unwrap_or(&vec![]).iter() {
    println!("Right side element: {} of type {}", e.value, e.element_type);
  }
 

   println!("Calculate : {} , answer is : {} " ,"-5 + 4*67y + 90 - 90+9/2", calculate_pemdas("-5 + 4*67 + 90 - 90+9/2 = 92").left);


}
