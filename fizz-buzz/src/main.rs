fn main() {
    do_fizz_buzz(100);
}

fn fizz_buzz(number: i32) -> String {
    let fizz = if is_divisible(number, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(number, 5) { "buzz" } else { "" };

    if fizz.is_empty() && buzz.is_empty() {
        return format!("{number}");
    }

    return format!("{fizz}{buzz}");
}

fn is_divisible(numerator: i32, denominator: i32) -> bool {
    return numerator % denominator == 0;
}

fn do_fizz_buzz(number: i32) {
    for i in 1..=number {
        println!("{}", fizz_buzz(i));
    }
}
