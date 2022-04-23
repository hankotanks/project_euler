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

pub(crate) fn euler6() -> u32 {
    let mut sum = 0;
    let mut square_sum = 0;

    for i in 1..101 {
        sum += i;
        square_sum += i * i;
    }

    sum * sum - square_sum
}

pub(crate) fn euler7() -> u32 {
    let mut count = 1;
    let mut current = 2;

    while count != 10001 {
        current += 1;
        if prime(current) {
            count += 1;
        }
    }

    current
}

// Originally, I was using an O(n) function to check primes
// But problem #10 took far too long
// This function is O(sqrt(n)) and uses the trial division method
// https://en.wikipedia.org/wiki/Primality_test
fn prime(num: u32) -> bool {
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

pub(crate) fn euler8() -> usize {
    let num = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    let mut product = 1usize;
    for i in 0..(num.len() - 13) {
        let mut current = 1usize;
        for j in 0..13 {
            current *= num[(i + j)..=(i + j)].parse::<usize>().unwrap();
        }

        if current > product {
            product = current;
        }
    }

    product
}

// Uses sum-relation method
pub(crate) fn euler9() -> usize {
    let mut a = 3usize;
    let mut b = 4usize;
    let mut c = 5usize;

    let mut m = 2usize;

    'outer: loop {
        for i in 1..m {
            a = m * m - i * i;
            b = 2 * m * i;
            c = m * m + i * i;

            if a + b + c == 1000 {
                break 'outer;
            }
        }

        m += 1;
    }

    a * b * c
}

// This function is O(n ^ 2)
// prime check should be optimized
pub(crate) fn euler10() -> usize {
    let mut total = 2usize;

    for i in 3..2000000 {
        if prime(i) {
            total += i as usize;
        }
    }

    total
}