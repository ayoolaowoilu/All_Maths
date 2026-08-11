use crate::equations::{ pemdas::calculate_pemdas};
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
 

    let result = calculate_pemdas("-5 + 4*67 + 90 - 90+9/2 = 92");
    println!(
        "Calculate: {} | left = {} | right = {:?} | is_equation = {}",
        "-5 + 4*67 + 90 - 90+9/2 = 92",
        result.left,
        result.right,
        result.is_equation,
    );

    let result2 = calculate_pemdas("(6-4)^(2/3) + 2/90");
    println!(
        "Calculate: {} | left = {} | right = {:?} | is_equation = {}",
        "(6-4)^(2/3)",
        result2.left,
        result2.right,
        result2.is_equation,
    );
}


