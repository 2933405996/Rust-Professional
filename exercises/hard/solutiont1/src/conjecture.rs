fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

pub fn goldbach_conjecture() -> String {
    let mut result: Vec<i32> = Vec::new();
    let mut n = 9;
    while result.len() < 2 {
        if !is_prime(n) {
            let mut flag = false;
            for i in 1..n {
                if is_prime(n - 2 * i * i) {
                    flag = true;
                    break;
                }
            }
            if !flag {
                result.push(n);
            }
        }
        n += 2;
    }
    format!("{},{}", result[0], result[1])
}
