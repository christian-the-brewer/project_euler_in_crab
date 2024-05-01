//By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms

fn main() {
    println!("{}", even_fibs(4_000_000));
}

fn even_fibs(target: i32) -> i32 {
    let mut sum = 0;
    let mut previous_term = 1;
    let mut current_term = 2;
    let mut temp: i32;

    while current_term < target {
        if current_term % 2 == 0 {
            sum += current_term;
        }
        temp = current_term;
        current_term += previous_term;
        previous_term = temp;
    }
    sum

}
