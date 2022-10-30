
use core::num;
use std::io::{self, stdin};
use rand::{self, Rng};

fn main() {
    println!("Hello, world!");

    const PI : f32 = 3.12;

    let  x;
    x = 4;
    // PI = 1;

    print!("{}", x);

    let x = "Tom";

    print!("{}", x);


    let arr : [i32; 3] = [1,2,3];

    // println!("{}", x);
    println!("{:?}", arr);
    println!("{}", arr[0]);


    let a: [i32; 4] = [1, 2, 3, 4]; // Parent Array

    let b: &[i32] = &a; // Slicing whole array
    let c = &a[0..4];
    println!(" c = {:?}", c);


    let mut name = String::new();
    name.push('1');
    name.push_str("aaaaa");
 

    println!("name = {}", name);


    name = String::from("aaaa"); // String
    
    let b = "This is the non-panicking alternative to ind";  // Slice String

    // let ch = 1;
    for ch in 0..10 {
        match ch % 2 {
           1 => println!("so chan"), 
           _ =>  println!("so le"), 
        }
    }

    let mut index = 0;
    loop {
        // println!("{}", index);
        index = index + 1;
        if index == 10 {
            break;
        }
    }

    let input = "This is the non-panicking alternative to ind";

    let mut reverse_input = String::with_capacity(input.len());

    for ch in input.chars() {
        reverse_input.insert(0, ch)
    }

    println!("Reverse String: {}", reverse_input);

    loop {
        
        println!("Nhap 1 so: ");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        // println!("Da nhap = {}", line);

        line.pop(); // \r\n   \n

        let number = line.parse::<i32>().unwrap();

        let rnd = rand::thread_rng().gen_range(0..100);

        if number == rnd {
            println!("Chuc mung ban!");
        } else {
            println!("Chuc ban may man lan sau!, Ket qua = {}", rnd);
        }
    }


}

