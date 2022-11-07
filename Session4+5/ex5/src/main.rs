// *********************************************
// ************* ALREADY FIXED !!! *************
// *********************************************


/////////////////////////////////////
// Bài 1
// Sửa code để compile thành công liên quan tới vấn đề lifetime
/////////////////////////////////////

// use std::io;
// fn main() {
//     let mut input: Vec<String>; // Fix: Convert to 'String' type. Avoid 'reference' (&)
//     loop {
//         let mut input_text = String::new();
//         println!("Type instruction in the format Add <name> to <department>:");
//         io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
//         let trimmed_text: String = input_text.trim().to_string();
//         input = trimmed_text.split(" ").map(String::from).collect(); // Fix: Convert to ownership, use 'map(String::from)' method
//         if input[0] == "Add" && input[2] == "to" {
//             break;
//         } else {
//             println!("Invalid format.");
//         }
        
//     }
//     println!("{:?}", input);
// }



/////////////////////////////////////
// Bài 2
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////


// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for String {
//     //Add your code here
//     // Start
//     // Copy the function structure as defined above
//     fn append_bar(self) -> String{
//         // Note: Returning 'self.push_str("Bar")' would return nothing. So this way won't work.
//         //format!("{}Bar", self) // First solution: Fix: use 'format' to concatenate
//         self + "Bar" // Second solution: Fix: use '+' operator in string to add more string together
//     }
// }
//     // Stop

// fn main() {
//     let s = String::from("Foo");
//     let s = s.append_bar();
//     println!("s: {}", s);
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_foo_bar() {
//         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
//     }

//     #[test]
//     fn is_bar_bar() {
//         assert_eq!(
//             String::from("").append_bar().append_bar(),
//             String::from("BarBar")
//         );
//     }
// }





/////////////////////////////////////
// Bài 3
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////



trait AppendBar {
    fn append_bar(self) -> Self;
}
//TODO: Add your code here
    // Start
// impl AppendBar for Vec<String> {

//     // Copy the function structure as defined above
//     fn append_bar(self) -> Vec<String>{
//         let mut s:Vec<String> = self;
//         s.push(String::from("Bar"));
//         s // This would return the whole vector 's'
//     }
// }
    // Stop

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_vec_pop_eq_bar() {
//         let mut foo = vec![String::from("Foo")].append_bar();
//         assert_eq!(foo.pop().unwrap(), String::from("Bar"));
//         assert_eq!(foo.pop().unwrap(), String::from("Foo"));
//     }
// }





/////////////////////////////////////
// Bài 4
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////
// The trait
////////////////////////////////////////////////////////////////////////////////

// trait Price {
//     fn price(&self, item_name: &str) -> Option<f32>;

//     fn total_price(&self, shopping_list: &[&str]) -> Option<f32>;
// }

// ////////////////////////////////////////////////////////////////////////////////
// // Store
// ////////////////////////////////////////////////////////////////////////////////

// struct Store {
//     name: String,
//     items: Vec<Item>,
// }

// #[derive(Debug)]
// struct Item {
//     name: &'static str,
//     price: f32,
// }

// impl Store {
//     fn new(name: String) -> Store {
//         Store {
//             name: name,
//             items: vec![],
//         }
//     }

//     fn add_item(&mut self, item: Item) {
//         self.items.push(item);
//     }
// }

// impl Price for Store {
//     fn price(&self, item_name: &str) -> Option<f32> {
//         for item in &self.items {
//             if item.name == item_name {
//                 return Some(item.price);
//             }
//         }
//         None
//     }
//     fn total_price(&self, shopping_list: &[&str]) -> Option<f32>{
//         let mut total_price:f32 = 0.0;
//         if shopping_list.is_empty(){
//             return None;
//         }
//         for item in shopping_list.iter(){
//             if self.price(item).is_none(){
//                 return None
//             }
//             else {
//                 total_price = total_price + self.price(item).unwrap();
//             }
//         }
//         return Some(total_price)
//     }
// }

// fn build_store() -> Store {
//     let mut store = Store::new(format!("Rustmart"));
//     store.add_item(Item { name: "chocolate", price: 5.0 });
//     store.add_item(Item { name: "socks", price: 23.0 });
//     store.add_item(Item { name: "plush Mozilla dinosaur", price: 13.0 });
//     store
// }

// ////////////////////////////////////////////////////////////////////////////////
// // Factory
// ////////////////////////////////////////////////////////////////////////////////

// // A factory for just a single kind of item
// struct Factory {
//     item_name: &'static str,
//     wholesale_price: f32,
// }

// impl Price for Factory {
//     fn price(&self, item_name: &str) -> Option<f32> {
//         if self.item_name == item_name {
//             Some(self.wholesale_price)
//         } else {
//             None
//         }
//     }

//     // Add 'total_price' function
//     fn total_price(&self, shopping_list: &[&str]) -> Option<f32> {
//         // Solution 1:
//         let mut total_price:f32 = 0.0;
//         if shopping_list.is_empty(){
//             return None;
//         }
//         for item in shopping_list.iter(){
//             if self.price(item).is_none(){
//                 return None
//             }
//             else {
//                 total_price = total_price + self.price(item).unwrap();
//             }
//         }
//         return Some(total_price)
        
//         // Solution 2:
//         // let mut total_price:f32 = 0.0;
//         // for item in shopping_list.iter(){
//         //     if let Some(cost) = self.price(item){
//         //         total_price += cost;
//         //     }
//         //     else {
//         //         return None;
//         //     }
//         // }
//         // return Some(total_price)
        
//         // Solution 3:
//         // let mut total_price:f32 = 0.0;
//         // for item in shopping_list.iter(){
//         //     match self.price(item){
//         //         Some(cost) => {
//         //             total_price += cost
//         //         }
//         //         _ => return None
//         //     }
//         // }
//         // Some(total_price)


//     }
// }

// fn build_factory() -> Factory {
//     Factory {
//         item_name: "sprocket",
//         wholesale_price: 7.67,
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////
// // Tests
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn total_price_store() {
//     let store = build_store();
//     let list = vec!["chocolate", "plush Mozilla dinosaur"];
//     assert_eq!(store.total_price(&list), Some(18.0));
// }

// #[test]
// fn total_price_missing_store() {
//     let store = build_store();
//     let list = vec!["chocolate", "plush Mozilla dinosaur", "fork and knife"];
//     assert_eq!(store.total_price(&list), None);
// }

// #[test]
// fn total_price_factory() {
//     let factory = build_factory();
//     let list = vec!["sprocket"];
//     assert_eq!(factory.total_price(&list), Some(7.67));
// }

// #[test]
// fn total_price_missing_factory() {
//     let factory = build_factory();
//     let list = vec!["sprocket", "socks"];
//     assert_eq!(factory.total_price(&list), None);
// }





