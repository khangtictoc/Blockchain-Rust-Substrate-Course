// fn vec_diff(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
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
//     let diff = vec_diff(&v2, &v1);

//     //let v3 = vec![3,4];
//     let diff2  = vec_diff(&diff, &v1);

//     println!("{:?}", all(diff, 3));

//     // println!("diff:{:?}",diff);
// }




// fn main(){

// }

// pub fn iter_num(num: i32) -> bool {

//     let num_str = num.to_string();
//     let chars = num_str.chars(); // <-- move occurs because `chars` has type `Chars<'_>`, which does not implement the `Copy` trait
//     // let len = chars.as_str().len();     // <-- `chars` moved due to this method call
//     let len = chars.count();
//     println!("Len = {:?}", len);

//     for c in chars {             // <-- ❌ "value used here after move": chars
//         println!(">>> {:?}", c);
//     }
//     // for c in num_str.chars() {             // <-- ❌ "value used here after move": chars
//     //     println!(">>> {:?}", c);
//     // }

//     return true;
// }


// fn main() {
//     let mut v = vec![1, 2, 3];

//     go(&mut v);

//     // still need v here, so I can't pass ownership to the "go' method above
//     println!("{}", v.len())
// }

// fn go(v: &mut Vec<i32>) {

//     for i in v.iter() {

//         println!("{}", i);

//     }
//     v.push(4);
// }


// Student: tự định nghĩa , tập hợp các thuộc tính mà mình định nghĩa : object
// student_a: khởi tạo đối tượng / instance( ví dụ), trường hợp cụ thể của đối tượng
// name, age, class: thuộc tính


// fn main() {
//     let student_a = Student { name: "John".to_string(), age: 20, class: "B1".to_string()};

//     let mut student_b = Student { name: "Dung".to_string(), age: 30, class: "B2".to_string()};

//     let name_a = student_a.get_name();
//     // let name = self.get_name();
//     // println!("name:{}",name_a);

//     println!("student_a:{:?}",student_a);
//     let new_student = Student::new("Dave".to_string(), 20, "E".to_string());
//     Student::print_name();
//     println!("{:?}",new_student);

//     student_b.update_class("D".to_string());
    
//     println!("class:{}", student_b.class);


//     let s = "Hello".to_string();
//     let convert_byte = s.as_bytes();

//     println!("s:{}",s);

//     let pos = Position::One;

//     let ipv4 = IP::IPV4("127.0.0.1".to_string());
//     println!("ipv4:{:?}",ipv4.get_name());
    
//     // match ipv4 {
//     //      IP::IPV4(x)=> println!("x:{}",x) ,
//     //     _ => println!("Ko the lay")
//     // }

//     // let ipv4 = IP::IPV4 { ip: "127.0.0.1".to_string() };

//     // match ipv4 {
//     //      IP::IPV4{ip}=> println!("x:{}",ip) ,
//     //     _ => println!("Ko the lay")
//     // }

//     let new_pos = Position::new();

//     println!("new_pos:{:?}",new_pos);

//     let a = Some(5);
//     let b: Option<u32> = None;

//     let res = a.unwrap();
//     println!("res: {}",res);

//     let res1 = match a {
//         Some(i) => i,
//         None => 0,
//     };

//     if let Some(value) = a {
//         println!("value:{}", value);
//     }
//     else {
//         println!("ko co");
//     }

//     let mut a: Vec<u8> = Vec::new(); //1.Sử dụng new() method
//     let mut b: Vec<u8> = vec![]; //2. Sử dụng vec! macro

//     let mut vec_test = vec![1,2,3,4,5,6];


//     // duyệt 
//     // ownership
//     // for i in vec_test {
//     //     println!("Take ownership of the vector and its element {}", i);
//     // }

//     // shared reference
//     for i in &vec_test {
//         println!("A reference to {}", i);
//     }

//     for i in vec_test.iter() {
//         println!("A reference to {}", i);
//     }

    
//     println!("vec_test:{:?}",vec_test);

//     // mutable reference

//     for i in &mut vec_test {
//         *i +=1;
//         println!("A mutable reference to {}", i);
//     }

//     // for i in vec_test.iter_mut() {
//     //     *i +=1;
//     //     println!("A mutable reference to {}", i);
//     // }


//     for mut i in vec_test.into_iter() {

//         i = i+2;
//         println!("A mutable reference to {}", i);
//     }


    
    

// }


// impl Student {
//     // hàm khởi tạo , contructor
//     fn new(name:String ,age: u8, class: String) -> Student {
//         return Student {name, age, class};
//     }

//     // fn new() -> Self {
//     //     return Student {name:"".to_string(), age:0, class:"".to_string()};
//     // }

//     // share reference
//     fn get_name(&self) -> String {
//         self.name.to_string()
//         //student_a.name
//     }

//     // ownership
//     //method
//     // &String -> &str
//     fn get_name_test(self) -> String {
//         self.name
//         //student_a.name
//     }

//     // mutable reference

//     fn update_class(&mut self, class_name:String) {
//         self.class = class_name;
//     }
//     // associated function
//     fn print_name() {
//         println!("My name is :David");
//     }




// }

// #[derive(Debug)]
// //abstract
// pub struct Student {
//     pub name: String,
//     pub age: u8,
//     pub class: String,
// }

// #[derive(Debug)]
// // options 
// enum Position{
//     One,
//     Two,
//     Three,
// }

// impl Position {
//     fn new() -> Position {
//         Position::One
//     }


// }
// #[derive(Debug)]
// enum IP {
//     IPV4(String),
//     IPV6(String),
// }

// impl IP {
//     fn get_name(self) -> String {
//         let res = match self {
//             IP::IPV4(x)=> x ,
//             _ =>"".to_string()
//         };

//         res
//     }
// }


// //struct lồng struct 

// // unit struct

// #[derive(Debug)]
// pub struct Double;

// #[derive(Debug)]
// // options 
// enum Position2{
//     One(Double),
//     Two,
//     Three,
// }


// #[derive(Debug)]
// enum IPVersion2 {
//     IPV4(Position2),
//     IPV6(String),
// }



// generic type : kiểu chung, kiểu tổng quát 

// use std::any::TypeId;
// use std::fmt::Debug;
// fn main(){

//     let res_1 = get(10u16);
//     let res_2 = get(10u8);
//     let res_3 = get("Hello".to_string());


//     let point1 = Point {x:16u32, y:30u16};

//     point1.get_x();
//     point1.get_y();



// }

// // placeholder 
// fn get<T>(x:T) -> T{
//     x
// }


// fn check<T: Debug+ 'static,U:Debug>(x:T, y:U){

//     if TypeId::of::<T>() == TypeId::of::<String>() {
//         println!("string={:?}", x)
//     } else {
//         println!("value={:?}", y)
//     }

// }


// Công dụng
// + rút gọn code
// + dễ dọc  -> khó đọc
// + kiểu tổng quát 



// struct Point<T,U> {
//     x:T,
//     y:U,
// }

// impl<T,U> Point<T,U> {
//     fn get_x(&self) ->&T {
//         &self.x
//     }
//     fn get_y(&self) ->&U {
//         &self.y
//     }

// }


// fn main() {
//     let mut shopping_list: Vec<&str> = Vec::new();
//     shopping_list.push("milk");
// }


// sử dụng generic type 
// fn main(){

// }

// struct Wrapper<T> {
//     value: T,
// }
// impl<T> Wrapper<T> {
//     pub fn new(value: T) -> Self {
//         Wrapper { value }
//     }
// }





// #[derive(Debug)]
// enum Message {
//     // TODO: define the different variants
//     Move{x:u32, y:u32},
//     Echo(String),
//     ChangeColor(u32,u32,u32),
//     Quit,

    
// }

// fn main() {
//     let messages = [
//         Message::Move { x: 10, y: 30 },
//         Message::Echo(String::from("hello world")),
//         Message::ChangeColor(200, 255, 255),
//         Message::Quit,
//     ];

//     for message in &messages {
//         message.call();
//     }
// }



// impl Message {
//     fn call(&self) {
//         println!("{:?}", &self);
//     }
// }






// fn print_number(maybe_number: Option<u16>) {
//     println!("printing: {}", maybe_number.unwrap());
// }

// fn main() {
//     print_number(Some(13));
//     print_number(Some(99));

//     let mut numbers: [Option<u16>; 5] = [Some(0);5];
//     for iter in 0..5 {
//         let number_to_add= Some(
//             ((iter * 1235) + 2) / (4 * 16)
//         );

//         numbers[iter as usize] = number_to_add;
//     }
// }



// // Fill in the blanks to make it work
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// // fn generic<char>(_s: SGen<char>) {}
// fn main() {
//     // Using the non-generic functions
//     reg_fn(S(A));          // Concrete type.
//     gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(5)); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(SGen('a'));

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen(5));

//     println!("Success!");
// }




// Fix the errors to make the code work.
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{x: 5.0, y: 10.0};
    println!("{}",p.distance_from_origin());
}










