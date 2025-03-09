fn main() {
    let temp = convert_temperature("celcius", 55.0);
    println!("{temp}");
    fibonacci_number(14);
}

fn convert_temperature(temp_type: &str, value: f64) -> f64 {
    if temp_type == "celcius" {
        (value - 32.0) / 1.8
    } else {
        (value * 1.8) + 32.0
    }
}

fn fibonacci_number(n: u16) {
    let mut num = 0;
    let mut ele = 1;
    print!("{num} ");
    print!("{ele} ");

    for _ in 2..n {
        let temp = ele;
        ele = ele + num;
        num = temp;
        print!("{ele} ");
    }
}
