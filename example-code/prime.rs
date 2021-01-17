fn is_prime(num: &i32) -> bool {
    if *num < 2 {
        return false;
    }
    let half: i32 = *num/2;
    let mut count: i32 = 2;

    while count <= half {
        if (*num % count) == 0 {
            return false;
        }
        count = count + 1;
    }
    return true;
}


fn print_n_prime(n: &mut i32) -> () {
    let mut count: i32 = 1;
    while *n > 0 {
        if is_prime(&count) {
            print(count);
            *n = *n - 1;
        }
        count = count + 1;
    }
}


let mut number: i32 = 10;
print_n_prime(&mut number);

