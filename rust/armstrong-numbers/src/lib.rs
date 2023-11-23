pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }
    let mut remainder = num;
    let num_digits = num.ilog10() + 1;
    let mut sum = 0;
    while remainder > 0 && sum <= num {
        let last_digit = remainder % 10;
        remainder /= 10;
        sum = match sum.checked_add(last_digit.pow(num_digits)) {
            // We can always raise `last_digit` to `num_digits` as max `num_digits` will be 10 and 9^10 is less than 2^32
            // Max `num_digits` is 10 because 2^32 is 10 digits
            Some(x) => x,
            None => return false,
        }
    }
    num == sum
}
