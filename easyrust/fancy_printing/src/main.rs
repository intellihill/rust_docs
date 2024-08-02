fn main() {
    // general programming print
    print!("c:\\mydesk\\new_drive");

    // hash raw text
    print!(r#"c:\mydesk\new_drive"#);

    println!("Let me tell you
        어떤 이야기를
    봅시다");

    // :? = debugging print
    // :p = point print 
    // :X = hex
    // :b = byte print
    // :
    let my_variable = &9;
    println!("{:p}", my_variable); // 0x561ccc334058
    
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each,
    // one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
