// Problem 1: Multiples of 3 and 5
// If we list all the natural numbers below 10 that are multiples of 
// 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
#[test]
fn test_pe1() {
    let mut sum = 0;
    for number in range(1i, 1000) {
        if number % 3 == 0 || number % 5 == 0 { 
            sum = sum + number 
        }
    }
    assert!(sum==233168);
}

#[test]
fn test_pe2() {
    let mut prev = 1i;
    let mut curr = 1;
    let mut sum = 0;
    while curr < 4 * 1000 * 1000 {
        if curr % 2 == 0 {
            sum = sum + curr;
        }
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    assert!(sum==4613732);
}

fn main() {
//    pe2();
    println!("{}",test_pe1());
    println!("{}",test_pe2());
}
