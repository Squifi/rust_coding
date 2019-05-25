fn main() {
    let number = 2139;
    println!("\n\nInitial Number is {}",number);

    // let desc = descending_order(number);
    // println!("result of desc: {}", desc);

    let vec_solution = using_vector(number);
    println!("Using vectors instead: {}", vec_solution);
}

fn descending_order(x: u64) -> u64 {
    let mut temp = x;
    let mut result = 0;
    let mut old_result = 0;
    let mut exp = 0;
    let mut current_largest = 0;

    while temp > 0 {
        let n = temp%10;
        let power = 10u64.pow(exp);
        println!("current power: {}", power);
        if n < current_largest {
            println!("{} is smaller than the result {}", n, current_largest);
            result = (result * 10) + n;
        } else if n < old_result {
            println!("{} is larger than the result {}", n, current_largest);
            result = n * (power/10) + result;
        } else {
            result = n * power + result;
            current_largest = n;
            println!("{} is highest Number so far", current_largest);
        }
        temp = temp/10;
        old_result = n;
        exp = exp + 1;
    }
    return result
}


fn using_vector(x: u64) -> u64 {
    let mut vec = Vec::new();
    let mut temp = x;
    let mut size = 0;

    while temp > 0 {
        let digit = temp%10;
        vec.push(digit);
        temp = temp/10;
        size = size + 1;
    }
    vec.sort();
    let mut y: u64 = 0;

    for x in 0..size {
        println!("this is {}", x);
        let pow = 10u64.pow(size);
        println!("Current power:{} -> vec[0] {}", pow, vec[0]);
        y = vec[2] * pow + y;
        size = size - 1;
    }
    y
}

