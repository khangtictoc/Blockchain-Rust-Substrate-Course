// *********************************************
// ************* ALREADY FIXED !!! *************
// *********************************************

///////////////////////////////////////////
// BAI 1
// Yêu cầu :
// + Sửa code liên quan tới vấn đề generic type (thay đổi ở định nghĩa struct)
///////////////////////////////////////////


// struct Point<T, U> { // Add another type 'U' to this
//     x: T,
//     y: U, // Change type 'T' to 'U'
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: 5, y : "hello".to_string()}; // `y` receives as another type, modify arbitrary generic type

//     println!("Success!");
// }



///////////////////////////////////////////
// BAI 2
// Yêu cầu :
// + Implement hàm sum dưới đây, sao cho việc kiểm tra assert_eq đúng 
///////////////////////////////////////////




// Implement the generic function below.

// use std::ops::Add; // Use trait `Add` as the error hinted

// // Use the syntax as below. <The reference source I have used: https://www.geeksforgeeks.org/rust-generic-function/>
// fn sum<T:Add<Output=T> + Copy >(a:T, b:T) -> T{ // Define 'sum' method as required with 2 given parameters
//     return a + b; // Return the sum of two operand
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }




///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////


// fn main() {
//     let a = A {p: Some("p".to_string())};
//     a.a();
// }

// struct A {
//     p: Option<String>
// }


// impl A {
//     fn a(self) { // Delete `Self` in return value, we don't need it. 
//     // We don't update `self` instance, we just need method `b` to print
//         Self::b(&self.p.unwrap());
  
//     }
//     fn b(b: &str) {
//         print!("b: {}", b)
//     }
// }





///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////

// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }

// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();

//     // Just for testing
//     println!("{}", x);
//     println!("{}", y);
// }


// impl MyData {
//     pub fn get_val1(&self) -> &i32 { // Use "" Share reference instead "". Add '&' before `self` and `i32`
//         return &self.val1; // Should not use clone here 
//     }

//      // Also fixe this method in similar way. This would allow us to print the value for checking
//     pub fn get_val2(&self) -> &String {
//         return &self.val2
//     }

//     pub fn get_both(&self) -> (&i32, &String) {  // Use "" Share reference instead "". Add '&' before `self` and `i32`
//         return (&self.val1, &self.val2); // Add '&' before `val1` and `val2`
//     }
// }


// *** TEST FREELY ON YOUR OWN ***
// fn main(){
//     let a:Option<&str> = Some("khang");
//     let b = &a.unwrap();
//     println!("{:p}", b);

// }


