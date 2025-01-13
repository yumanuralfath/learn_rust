mod codewars;
mod leetcode;

fn main() {
    let result = codewars::grasshopper::check_for_factor(10, 2);
    println!("{}", result);
}

// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// pub fn calculate_area(height: f64, width: f64) -> f64 {
//     height * width
// }

// // pub fn mutating_variables() -> &'static str {
// //     let mut text = "hello";
// //     text = "world";
// //     text
// // }

// const MAX_SIZE: i32 = 100;
// pub fn constant() -> i32 {
//     MAX_SIZE
// }

// //
// pub fn numerical_type_conversion(n: i32) -> u32 {
//     n as u32
// }

// pub fn math_operations(a: i32, b: i32) -> (i32, i32, i32, i32) {
//     let sum = a + b;
//     let difference = a - b;
//     let product = a * b;
//     let quotient = a / b;
//     (sum, difference, product, quotient)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_calculate_area() {
//         let area = calculate_area(2.0, 3.0);
//         assert_eq!(area, 6.0);
//     }

//     #[test]
//     fn test_constant() {
//         assert_eq!(constant(), 100);
//     }

//     // #[test]
//     // fn test_mutating_variables() {
//     //     assert_eq!(mutating_variables(), "world");
//     // }

//     #[test]
//     fn test_data_types() {
//         use std::any::TypeId;
//         assert_eq!(TypeId::of::<u8>(), TypeId::of::<u8>());
//     }

//     #[test]
//     fn test_numerical_type_conversion() {
//         assert_eq!(numerical_type_conversion(1), 1);
//         assert_eq!(std::any::TypeId::of::<u32>(), std::any::TypeId::of::<u32>());
//     }

//     #[test]
//     fn test_math_operations() {
//         assert_eq!(math_operations(1, 2), (3, -1, 2, 0));
//     }
// }
