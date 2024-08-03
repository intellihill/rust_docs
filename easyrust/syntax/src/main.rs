fn main() {
    reference()

    let country = "대한민국".to_string();
    print_country(&country);
    print_country(&country);
    print_country(&country);

    add_is_great(&mut my_country);
    add_is_great(&mut my_country);

    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    uninitialized()

}

// & immutable reference / shared reference
// &mut mutable reference / unique refernece

// & = reference
// * = oposite reference
// & -> * 
// && -> **

fn reference() {
    let mut my_number = 9;
    let num_ref = &mut my_number;

    // num_ref = 10; // impossible
    *num_ref = 10; // possible

    println!("Number is now {}", my_number);
}

// OWNERSHIP
// move semantics 

fn print_country(country_name: &String) {
    println!("My country is {}", country_name);
}

// mutable reference in functions

fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
}

// Ownership and copy types
// It's trivial to copy the bytes 간단한 타입은 소유권이 필요가 없는듯?

fn prints_number(number: i32) {
    println!("{}", number);
}

// uninitialized variable
// control flow

fn uninitialized() {
    // possibly uninitialized = maybe doesn't have a value yet
    // let my_number: u8;
    let my_number = {
        // 복잡한 것들
        let x = loop_then_return(x);
        my_number = x
    }
    println!("{}", my_number);
}

fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}
