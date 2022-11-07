// fn main() {
//     let mut counter = Counter{x:0};

//     counter.next();
//     counter.next();
//     counter.next();

//     println!("Counter:{:?}",counter);


//     let mut vector = vec![3,4,2];
//     println!("0th element :{:?}", vector.next());
//     println!("1th element :{:?}", vector.next());
//     println!("2th element :{:?}", vector.next());
//     println!("3th element :{:?}", vector.next());
//     println!("4th element :{:?}", vector.next());

//     // let vector = vec![5,2,3];
//     // let mut iter = vector.iter();
//     // println!("{:?}",iter);
//     // println!("value:{:?}", iter.next());
//     // println!("value 2 :{:?}", iter.next());
//     // println!("{:?}",iter);
//     // println!("value 3 :{:?}", iter.next());
//     // println!("value 3 :{:?}", iter.next());
// }



//generic gắn với trait
pub trait Count<T> {

    fn next(&mut self) -> T;
}

#[derive(Debug)]
struct Counter {
    x: u64
}

impl Count<u64> for Counter {
    fn next(&mut self) -> u64 {

        self.x = self.x +1;
        self.x
    }
}



pub trait Iterator1 {
    type Item;// associated type (nó cũng như là generic type)
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator1 for Vec<u32> {
    type Item = u32;

    // vec = vec[5,2,3]
    //vec.next() -> Some(5)
    //vec.next() -> Some(2)
    //vec.next() -> Some(3) 
    // vec.next() -> None
    fn next(&mut self) -> Option<u32>{
        
        if !self.is_empty() {
            let res = self.remove(0);
            Some(res)
        }
        else {
            None
        }
        

        
    }
}

// pub trait Iterator2<T> {
//     fn next(&mut self) -> Option<T>;
// }


// impl Iterator2<u32> for Vec<u32> {

//     // vec = vec[5,2,3]
//     //vec.next() -> Some(5)
//     //vec.next() -> Some(2)
//     //vec.next() -> Some(3) 
//     // vec.next() -> None
//     fn next(&mut self) -> Option<u32>{
        
//         if !self.is_empty() {
//             let res = self.remove(0);
//             Some(res)
//         }
//         else {
//             None
//         }
        

        
//     }
// }


// use std::ops::Add;
// use std::fmt::Display;
// use std::fmt::Debug;

// // trait bound -> constraint behavior 
// // implemem trait -> sử dụng dc các behavior nhưng với điều kiện là trait đó phải implement cho 1 object cụ thể 
// fn sum1<T:std::ops::Add+ Add<Output = T>> (x: T, y : T) -> T {
//     x+y
// }


// fn sum2<T:std::ops::Add+ Add<Output = T>, U  > (x: T, y : T) -> T 
// where T: Display,
//     U: Debug
// {
//     x+y
// }

// #[derive(Debug, Clone)]
// struct Point {
//     x: i32,
//     y: i32,
// }


// impl Add for Point {
//     type Output = Point;
//     fn add(self, rhs: Self) -> Point{
//         Point {
//             x: self.x + rhs.x,
//             y:self.y + rhs.y
//         }
//     }
// }

// fn main(){
//     let point1 = Point{x:1,y:2};
//     let point2 = Point{x:1,y:2};  

//     // alias  chuyển từ kiểu dữ liệu primitive (Struct) -> sang kiểu dự liệu custom
//     let point= point1.clone() + point2.clone();

//     println!("point:{:?}", point);


//     let res = sum1(3i32,4i32);
//     let re1 = sum1(3u32,4u32);

//     let res3 = sum2(point1, point2);

//     // <Point as Add<Point>>::Output 

//     // nó đang chuyển kiểu Point sang kiểu Output của trait Add

//     // Point (struct ) -> Self::Output (của trait Add)
//     // point1 : Point , point2: Point , point : Point
//     // output : Point struct 
// }

// // point1 = {1,2} + point2 = {2,3} -> point1 + point2 -> {3,5}
// // struct + struct 

// // std -> mình phải tự implement



// use std::fmt::Display;

// struct Test {
//     x: u32
// }
// fn main(){

//     let test = Test {x:10};

//     println!("x:{}",test);
// }

// // primitive type : u32, float, .. integer 
// impl Display for Test{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(Test: {})", self.x)
//     }
// }


// struct Container(i32,i32);


// // impl Container {
// //     fn diff(&self) -> i32 {
// //         self.0 - self.1
// //     }
// // }


// trait Contains<A, B> {

//     fn contains(&self, _:&A, _: &B) -> bool;

//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }
// impl Contains<i32, i32> for Container {
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool{
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }

//     fn first(&self) -> i32{
//         self.0
//     }
//     fn last(&self) -> i32{
//         self.1
//     }
// }


// // ý là mình phải constraint C  trở thành 1 traits -> sử dụng dc các function mà trait đó định nghĩa 
// fn difference<A, B, C>(container: &C) -> i32  where C: Contains<A,B>{

//     container.first() - container.last()
    
// }

// impl Contains<i32, i32> for Point {
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool{
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }

//     fn first(&self) -> i32{
//         self.1
//     }
//     fn last(&self) -> i32{
//         self.0
//     }
// }


// pub struct Point(i32,i32);

// fn main(){
//     let container = Container(10,20);
//     let point = Point(20,30);
//     let res = container.contains(&10, &20);

//     let first = container.first();
//     let last = container.last();
//     println!("bool:{}",res);
//     println!("first:{}",first);
//     println!("last:{}",last);

//     let result = difference(&container);
//     let result2 = difference(&point);
// }

// struct Container(i32,i32);

// trait Contains{
//     type A;
//     type B;
//     fn contains(&self, _:&Self::A, _: &Self::B) -> bool;

//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }


// // Associated type
// impl Contains for Container {
//     type A = i32;
//     type B =i32;
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool{
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }

//     fn first(&self) -> i32{
//         self.0
//     }
//     fn last(&self) -> i32{
//         self.1
//     }
// }


// // alias type 
// // type AB = A+ B;


// fn difference<C>(container: &C) -> i32  where C: Contains{

//     container.first() - container.last()
    
// }







// struct Container2(i32,i32);

// trait Contains2<A, B>{

//     fn contains2(&self, _:&A, _: &B) -> bool;

//     fn first2(&self) -> i32;
//     fn last2(&self) -> i32;
// }


// // Associated type
// impl Contains2<i32, i32> for Container2 {

//     fn contains2(&self, number_1: &i32, number_2: &i32) -> bool{
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }

//     fn first2(&self) -> i32{
//         self.0
//     }
//     fn last2(&self) -> i32{
//         self.1
//     }
// }


// // alias type 
// // type AB = A+ B;


// fn difference2<C>(container: &C) -> i32  where C: Contains{

//     container.first() - container.last()
    
// }



// // ưu điểm 
// // + rút gọn khi nhìu generic type (trait bound)
// // +  tại sao lại sử dụng associated type 

// fn main(){

// }
// // fn main(){

// //     let container = Container(10,20);

// //     let res = container.contains(&10, &20);

// //     let first = container.first();
// //     let last = container.last();
// //     println!("bool:{}",res);
// //     println!("first:{}",first);
// //     println!("last:{}",last);

// //     let result = difference(&container);
// // }



// Generic parameter ->static dispatch
//and trait object -> dynamic dispatch





// trait Processor {
//     fn compute(&self, x: i64, y: i64) -> i64;
//  }
 
//  struct Risc {}
 
//  impl Processor for Risc {
//     fn compute(&self, x: i64, y: i64) -> i64 {
//         x + y
//     }
//  }
 
//  struct Cisc {}
 
//  impl Processor for Cisc {
//     fn compute(&self, x: i64, y: i64) -> i64 {
//         x * y
//     }
//  }


//  struct Kisc{}
//  // generic parameter -> static dispatch 
//  // trait bound , constraint
//  fn process<P: Processor>(processor: &P, x: i64) {
//     let result = processor.compute(x, 42);
//     println!("{}", result);
//  }
 

//  //tổng quát (generic type ) -> 1 type -> nhiều type mong muốn tuỳ thuộc vào ngữ cảnh
//  // Monomorphization cho trait 


//  fn process_risc(processor: &Risc, x: i64) {
//     let result = processor.compute(x, 42);
//     println!("{}", result);
//  }

//  fn process_cisc(processor: &Cisc, x: i64) {
//     let result = processor.compute(x, 42);
//     println!("{}", result);
//  }

//  fn process_kisc(processor: &Kisc, x: i64) {
//     let result = processor.compute(x, 42);
//     println!("{}", result);
//  }


//  pub fn main() {
//     let processor1 = Cisc {};
//     let processor2 = Risc {};
 
//     process(&processor1, 1);
//     process(&processor2, 2);
//  }
 







trait Processor {
    fn compute(&self, x: i64, y: i64) -> i64;
 }
 
 struct Risc {}
 
 impl Processor for Risc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x + y
    }
 }
 
 struct Cisc {}


 // compile dc khi mà kiểu dữ liệu biết dc size 
 
 impl Processor for Cisc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x * y
    }
 }


  fn process(processor: Box<dyn Processor>, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
 }


 fn main(){

    // Box : smart pointer
    let processor1 = Box::new(Cisc {});
    let processor2 = Box::new(Risc {});
 
    process(processor1, 1);
    process(processor2, 2);
}