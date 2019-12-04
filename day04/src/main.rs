use std::cmp::Ordering;

fn get_digits(value: u32) -> [u8; 6] {
    [
        (value / 10u32.pow(5) % 10) as u8,
        (value / 10u32.pow(4) % 10) as u8,
        (value / 10u32.pow(3) % 10) as u8,
        (value / 10u32.pow(2) % 10) as u8,
        (value / 10u32 % 10) as u8,
        (value % 10) as u8,
    ]
}

fn passes(value: u32) -> (bool, bool) {
    let digits = get_digits(value);

    let mut a = 0;
    let mut has_double = false;
    let mut has_exact_double = false;

    while a < digits.len() - 1 {
        let mut b = a + 1;

        while b < digits.len() {
            match digits[b].cmp(&digits[a]) {
                Ordering::Less => return (false, false),
                Ordering::Greater => break,
                Ordering::Equal => b += 1,
            }
        }

        has_double |= b - a >= 2;
        has_exact_double |= b - a == 2;

        a = b;
    }

    (has_double, has_exact_double)
}

fn main() {
    let mut count = 0;
    let mut count2 = 0;

    for i in 123257..=647015 {
        let (part_one, part_two) = passes(i);

        if part_one {
            count += 1;
        }

        if part_two {
            count2 += 1;
        }
    }

    println!("Part one: {}", count);
    println!("Part two: {}", count2);
}
