//find the sum of all muliples of 3 and 5 below 1000

fn main() {
    println!("The sum total is {}", find_sum(1000));
}

fn find_sum(target: i32) -> i32 {
    let mut sum = 0;
    for i in 1..target {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
