// shadowing 같은 이름을 다시 쓰는 것

fn main() {
    let x = 9;
    let x = double(x);
    let x = triple(x);
    
    println!("{}", x);
}

fn double(input: i32) -> i32 {
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}

