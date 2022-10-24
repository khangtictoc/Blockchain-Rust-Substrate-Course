// ******************************
// *****  ALL READY FIXED ! *****
// ******************************


//====================================
// BÀI 1:
// Yêu cầu: Hiện tại chưa cần quan tâm tới logic
// Mọi người thử fix lỗi khi dùng clone hoặc phương
// pháp borrowing sao cho biểu thức `diff` compile được và
// biểu thức in ra cũng compile được  

//=====================================


// Returns a new vector containing the element-wise difference between
// `left` and `right`. To be clear, this returns a vector `rv` such that
// for every `i`, `left[i] - right[i] == rv[i]`. Note that the implementation
// assumes that `left.len() == right.len()`.

// fn vec_diff(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> { // Add '&' before `left` and `right`
//     let mut rv = Vec::new();
//     for (l, r) in left.iter().zip(right.iter()) {
//         rv.push(l - r);
//     }
//     return rv;
// }

// /// Returns true if and only if all elements in `vec` are equal to `value`.
// fn all(vec: Vec<i32>, value: i32) -> bool {
//     for elem in vec.iter() {
//         if *elem != value {
//             return false;
//         }
//     }
//     return true;
// }

// fn main() {
//     let v1 = vec![0, 1, 2];
//     let v2 = vec![3, 4, 5];
//     let diff = vec_diff(&v2, &v1); //  Ownership of `v1` disappears when the function finishes ==> Add '&' before `v2`

//     let diff2  = vec_diff(&diff, &v1); // Ownership of `diff` disappears when the function finishes ==> Add '&' before `v1` and `diff`
//     println!("{:?}", all(diff, 3));
// }



//====================================
// BÀI 2:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

//=====================================


// fn main() {
//     let objetive = 3126.59;

//     // 27
//     let values: Vec<f64> = vec![
//         2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
//         139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
//         4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
//     ];

//     let values_number = values.len();
//     let values_index_max = values_number - 1;

//     let mut additions: Vec<usize> = vec![0];

//     println!("{:?}", values_number);

//     while additions.len() > 0 { 
//         let mut addition: f64 = 0.0;
//         let mut saltar: i32 = 0;

//         // Sumar valores en additions
//         for element_index in &additions {   // Ownership of `additions` disappears when the first 'for' loop runs.  ==> Add '&' in `additions`
//             let addition_aux = values[*element_index]; // `element_index` is currently a pointer. Deference with '*' before `element_index`
//             addition = addition_aux + addition;
//         }
//     }
// }



// ====================================
// BÀI 3:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================


// fn main(){

// }

// pub fn iter_num(num: i32) -> bool {

//     let num_str = num.to_string();
//     let chars = num_str.chars(); // <-- move occurs because `chars` has type `Chars<'_>`, which does not implement the `Copy` trait
//     let len = chars.clone().count();     // <-- `chars` moved due to this method call  ==> Use '.clone()' method

//     println!("Len = {:?}", len);

//     for c in chars{             // <-- ❌ "value used here after move": chars
//         println!(">>> {:?}", c);
//     }

//     return true;
// }



// ====================================
// BÀI 4:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================


// fn main() {
//     let mut v = vec![1, 2, 3];

//     go(&mut v);

//     // still need v here, so I can't pass ownership to the "go' method above
//     println!("{}", v.len())
// }

// fn go(v: &mut Vec<i32>) {
//     for i in v.iter() { // Use '.iter()' for each iteration of `i`
//         println!("{}", i);
//     }
//     v.push(4);
// }

// NOTE: THERE MUST BE AT LEAST ONE 'MAIN' FUNCTION FOR EACH EXECUTABLE FILE
