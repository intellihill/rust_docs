fn main() {
    // String = sized 
    // &str : ref str "string slice" dynamic type(heap memory)
    // owned type
    let my_name = "David".to_string(); 
    let other_name = String::from("David2");
    // growable + shrinkable
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{}", my_other_name);

    // String
        // .capacity
        // .push
        // .push_str
        // .pop
        // .with_capacity
    // method = .function

    // let mut my_name = "".to_string();
    let mut my_name = String::with_capacity(26);
    println!("Length is {} and Capacity is: {}", my_name.len() , my_name.capacity());
    my_name.push_str("David");
    println!("Length is {} and Capacity is: {}", my_name.len() , my_name.capacity());
    my_name.push('!'); // add char
    println!("Length is {} and Capacity is: {}", my_name.len() , my_name.capacity());
    my_name.push_str(" and I live in Seoul"); // add String
    println!("Length is {} and Capacity is: {}", my_name.len() , my_name.capacity());
    my_name.push('a');

}
