// ==================================================================================
// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ?
// (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
// let sub_arr = [6, 8, 10];
// ==================================================================================

fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let mut is_subarray = false;
    for mut i in 0..org_arr.len(){
        if &org_arr[i] == &sub_arr[0]{
            i += 1;
            let mut j = 1;
            while &org_arr[i] == &sub_arr[j] {
                i += 1;
                j += 1;
                if j == sub_arr.len(){
                    println!("This array IS a subarray of the given one");
                    is_subarray = true;
                    break;
                }
            }
        }
        if is_subarray == true{
            break;
        }
    }
    if is_subarray == false{
        println!("This array IS NOT a subarray of the given one");
    }
}
