use num::Complex;


fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

/// c가 망델브로 집합에 속하는지 아닌지를 판단하며, 결론 내리는 데 필요한 반복 횟수는 최대 limit회로 제한한다.
/// 
/// c가 망델브로 집합에 속하지 않으면 Some(i)를 반환하는데, 여기서 i는 c가 원점을
/// 중심으로 반경이 2인 원을 벗어나는 데 걸린 반복 횟수다. c가 망델브로 집합에 속하는 것 같으면
/// (좀 더 정확히 말해서 반복 횟수가 limit이 될 때까지도 c가 망델브로 집합에 속하지 않는다는 걸 입증하지 못하면) None을 반환한다.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
} 

fn main() {

}
