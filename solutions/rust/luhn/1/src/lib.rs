/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut number_count = 0;
    let mut sum = 0;
    for c in code.chars().rev() {
        if c == ' ' {
            continue;
        }
        let Some(mut num) = c.to_digit(10) else {
            #[cfg(debug_assertions)]
            {
                println!("Invalid digit found {c}");
                dbg!(code, number_count, sum);
            }

            return false;
        };
        number_count += 1;
        if number_count % 2 == 0 {
            // Double as it's a 2nd number from the right
            num *= 2;
            if num > 9 {
                num -= 9;
            }
        }
        sum += num;
        sum %= 10;
    }

    #[cfg(debug_assertions)]
    dbg!(code, number_count, sum);

    number_count > 1 && sum % 10 == 0
}
