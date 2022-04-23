pub(crate) fn euler1() -> u32 {
    let mut sum = 0u32;
    for i in 0..1000 {
        if i % 5 == 0 || i % 3 == 0 {
            sum += i;
        }
    }

    sum
}
pub(crate) fn euler2() -> u32 {
    let mut sum = 2;

    let mut n0 = 1u32;
    let mut n1 = 2u32;
    while n1 < 4000000 {
        let temp = n1;
        n1 = n1 + n0;
        n0 = temp;

        if n1 % 2 == 0 {
            sum += n1;
        }
    }

    sum
}

pub(crate) fn euler3() -> usize {
    return largest_prime_factor(600851475143);
}

fn largest_prime_factor(value: usize) -> usize {
    for i in 2..value {
        if value % i == 0 {
            return largest_prime_factor(value / i);
        }
    }

    value
}

pub(crate) fn euler4() -> u32 {
    let mut largest = 0;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            if palindrome(i * j) && i * j > largest {
                largest = i * j;
            }
        }
    }

    largest
}

fn palindrome(num: u32) -> bool {
    let places = num.to_string().len() as u32;
    for i in 0..((places + 1) / 2) {
        if get_digit(num, i) != get_digit(num, places - i - 1) {
            return false;
        }
    }

    true
}

fn get_digit(num: u32, place: u32) -> u32 {
    (num % 10u32.pow(place + 1)) / 10u32.pow(place)
}

pub(crate) fn euler5() -> u32 {
    let mut current = 1;
    'outer: loop {
        for i in 1..=20 {
            if current % i != 0 {
                current += 1;
                continue 'outer;
            }
        }

        break 'outer;
    }

    current
}