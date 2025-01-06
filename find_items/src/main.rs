// Topic: working with a Generics

// Program requirements:
// filters duplicates within a list(Vec(i32>)

// Notes:
// - Use a Generics 
// change unique() to accept a Vec<T>, where T is a type that implements Ord trait.

// - Implement unique()
// it accepts a Vec<i32> as an argument, 
// returning a Vec<i32> with no dublicates.

//   1. no dublicates in the input list.
// let list = vec![1,2,3];
// assert_eq!(unique(list), list);

//   2. dublicates in the input list.
//  let list = vec![1,2,2];
// assert_eq(unique(list), unique_function)


fn unique<T:Ord>(mut find_items: Vec<T>) -> Vec<T>{
    find_items.sort();
    find_items.dedup();
    find_items
}


fn main() {
    let input: Vec<i32> = vec![1,2,3,3];
    let answer: Vec<i32> = unique(input);
    println!("Unique items: {:?}", answer);
}
