
// fn main() {
//     let org_arr = [1, 2,3,5,6,8, 10, 11];
//     let sub_arr = [2,3,8];

//     let mut i = 0;
//     let mut j = 0;
//     let mut count =0 ;

//     while i < sub_arr.len() && j < org_arr.len(){

//         if org_arr[j] == sub_arr[i] {
//             i +=1;
//             j+=1;

//             count +=1;

//         }
//         else {
//             //i = i-j+1;
            
//             j = j-i +1 ;

//             i =0; 
            
//         }
//         println!("i:{}",i);
//         println!("j:{}",j);
//     }

//     if count == sub_arr.len(){
//         println!("True");
//     }
//     else {
//         println!("False");
//     }

// }


// fn main() {
//     let org_arr = [1, 2,3,5,6,8, 10, 11];
//     let sub_arr = [2,3,8];
//     let res = is_child(org_arr, sub_arr);

//     println!("res:{}",res);
//     }


// fn is_child(dad:[i32;8] ,child:[i32;3]) -> bool {
//     let mut i = 0;
//     let mut j = 0;
//     let mut count =0 ;

//     while i < child.len() && j < dad.len(){

//         if dad[j] == child[i] {
//             i +=1;
//             j+=1;

//             count +=1;

//         }
//         else {
//             //i = i-j+1;
            
//             j = j-i +1 ;

//             i =0; 
            
//         }
//     }

//     if count == child.len(){
//         return true;
//     }
//     else {
//         return false;
//     }
// }


// fn test(x: u32){

// }

// fn test_1(){

// }

// fn test_2(x:u32) -> u32{
//     0
// }


// fn main(){
//     let mut x = 5;
//     let y = x;
//     //x = x +1;
//     // pointer
//     let z = &mut x;

//     *z = *z+1;


    
//    // println!("x:{}",x);
//     println!("z:{:p}",z);
//     println!("y:{}",y);

//     let mut s1 = String::from("Hello");

//     // Borrowing/Reference
//     //let s2 = &s1;

//     // Mutable borrowing
//     let s2 = &mut s1;

//     s2.push_str("World");
//     // let s3 = &s1;

//     // let s4 = &s1;

//     // s1.push_str("World");

//     // println!("s2:{}",s2);





//     //println!("{}",s2);

//     //let s3 = s1;
//     // Ownership // drop   , move
//     //println!("s1:{}", s1);



//     // println!("len:{}",s1.len());
//     // println!("capacity:{}", s1.capacity());

//     // s1.push_str("World");
//     // println!("len:{}",s1.len());
//     // println!("capacity:{}", s1.capacity());

//     // {
//     //     let s3 = s2;
        
//     // }
//     // println!("s1:{}",s1);
//     // println!("s2:{}",s2);


// }


// // fn test() -> u32 {
// //     8i32
// // }



// // fn test() -> u32 {
// //     return 9;
// // }


// // fn test() -> u32 {
// //     9
// // }



// fn main() {
//     let s = String::from("hello");
//     // let x = 5;
//     // let res_x = change_2(&x);
//     let res = change_1(&s);

//     //Cách 1: clone 
//     //let s1= s;

//     //Cách 2: Borrowing/ Reference
// }

// fn change_1(some_string:&String) -> String {
//     // let mut s = String::from(some_string);
//     // s.push_str("Workd");
//     //some_string.push_str(", world");
//     some_string.to_string()
//     //s

// }


// fn change_2(some_string:&i32) -> i32 {
//     // let mut s = String::from(some_string);
//     // s.push_str("Workd");
//     //some_string.push_str(", world");
//     *some_string
//     //s

// }







// fn main(){
//     let mut s = String::from("hello world");
//     s.push_str("WOld");
//     let hello = &s[0..5];
//     let world = &s[6..11];

// }



//================================
//VÍ DỤ 1
//================================



fn main() {
    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn remove_vowels(name: String) -> String {
    // Goal #1: What is needed here to make this compile?
    let output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels
            }
            _ => {
                output.push(c);
            }
        }
    }
    output
}

fn print_out(name: String) {
    let devowelized_name = remove_vowels(name);
    println!("Removing vowels yields {:?}", devowelized_name);

    // Goal #2: What happens when you uncomment the `println` below?
    // Can you change the code above so that the code below compiles
    // successfully?
    //
    // println!("Removing vowels from {:?} yields {:?}",
    //          name, devowelized_name);

    // Extra credit: Can you do it without copying any data?
    // (Using only ownership transfer)
}



//================================
//VÍ DỤ 2
//================================




// pub fn main() {
//     let string = format!("my friend");
//     greet(string.clone());
//     greet(string);
// }

// fn greet(name: String) {
//     println!("Hello, {}!", name);
// }

// // Goal #1: Convert `greet` to use borrowing, not ownership, so that
// // this program executes without doing any cloning.
// //
// // Goal #2: Use a subslice so that it prints "Hello, friend" instead of
// // "Hello, my friend".


//================================
//VÍ DỤ 3
//================================




// pub fn main() {
//     let (mut str1, str2) = two_words();
//     str1 = join_words(str1, str2);
//     println!("concatenated string is {:?}", str1);
// }

// fn two_words() -> (String, String) {
//     (format!("fellow"), format!("Rustaceans"))
// }

// /// Concatenate `suffix` onto the end of `prefix`.
// fn join_words(mut prefix: String, suffix: String) -> String {
//     prefix.push(' '); // separate the words with a space
//     for ch in suffix.chars() {
//         prefix.push(ch);
//     }
//     prefix
// }

// Challenge: Convert `join_words` to use borrowing, not ownership.
// The new function should mutate `prefix` in place, and should not
// take ownership of `suffix`.
//
// Hint: If you'd like a hint as to how to proceed, open
// <http://rust-tutorials.com/exercises/hint/mutable_borrow_1/>.

// Question: Now that you've converted `join_words`, what happens if you
// call `join_words` using the same string for `prefix` and `suffix`?
// Why?




//================================
//VÍ DỤ 4
//================================



// fn main() {
    
//     let x = change_value(10,&mut 20);
// }
// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input ==1 {
//         *output =3;
//     }
//     else {
//         *output = 4;
//     }

//     *output
// }


//================================
//VÍ DỤ 5
//================================



// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 2;
//         if vector_is_prime(num, &primes) {
//             count += 1;
//             primes.push(num);
//         }
//     }
//     println!("{:?}", primes);

//     let array = &[10,20];
// }
// // p = &vec[1,2,3,4] -> &1, &2, &3

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
//     for &i in p {
//         if num > i && num % i != 0 {
//             return false;
//         }
//     }

//     true
// }

//================================
//VÍ DỤ 6
//================================


// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;
//     //let v1 = &v;
    
//     for n in v.iter() {
//         max = std::cmp::max(max, *n);
//     }
//     //println!("{:?}",v);
//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
    
//     for n in v {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }