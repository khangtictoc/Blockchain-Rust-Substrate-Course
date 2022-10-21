// =============================    EX 2   ==================================
// Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện 
// của từ đó trong chuỗi đã cho, và chuỗi không chứa ký tự nhập từ bàn phím. 
// Lưu ý: khong phân biệt viết hoa, viết thường
// Ví dụ: let input = “adbcdaDd”. 
// Nhập s = ‘a’ => in ra kết quả : 2, “dbcdDd”
// Nhập s = ‘d’ => in ra kết quả : 4, “abca”
// ==========================================================================

use std::io;
fn main() {
    let input = "adbcdaDd"; // Given string

    println!("Enter a character: ");
    let mut search_input = String::new();
    std::io::stdin().read_line(&mut search_input).unwrap(); // Get input from keyboard

    // Remove '\r' or '\n' at the end of string input (This depends on the using OS)
    let raw_search_input = search_input.replace(&['\r', '\n'][..], "");

    // println!("{:?}", raw_search_input);

    //Create uppercase and lowercase of input character
    let search_input_lower = raw_search_input.to_ascii_lowercase();
    let search_input_upper = raw_search_input.to_ascii_uppercase();

    // println!("{}", search_input_lower);
    // println!("{}", search_input_upper);

    // Remove all the case-insensitive character  
    let mut result = input.replace(&search_input_lower, "");
    result = result.replace(&search_input_upper, "");
    
    println!("{}", result);
}