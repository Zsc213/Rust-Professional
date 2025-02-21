/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

fn ma(m: &mut Vec<Vec<i32>>, n: &Vec<Vec<i32>>) {
    let temp = m.clone();
    m[0][0] = temp[0][0] * n[0][0] + temp[0][1] * n[1][0];
    m[0][1] = temp[0][0] * n[0][1] + temp[0][1] * n[1][1];
    m[1][0] = temp[1][0] * n[0][0] + temp[1][1] * n[1][0];
    m[1][1] = temp[1][0] * n[0][1] + temp[1][1] * n[1][1];
}

fn mb(m: &mut Vec<Vec<i32>>) {
    let temp = m.clone();
    m[0][0] = temp[0][0] * temp[0][0] + temp[0][1] * temp[1][0];
    m[0][1] = temp[0][0] * temp[0][1] + temp[0][1] * temp[1][1];
    m[1][0] = temp[1][0] * temp[0][0] + temp[1][1] * temp[1][0];
    m[1][1] = temp[1][0] * temp[0][1] + temp[1][1] * temp[1][1];
}

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    let mut res_matrix = vec![vec![1, 0], vec![0, 1]];
    let mut i: i32 = 0;
    let mut temp_matrix = vec![vec![0, 1], vec![1, 1]];
    let mut m = 1 << i;
    while m <= n {
        if m & n != 0 {
            ma(&mut res_matrix, &temp_matrix)
        }
        mb(&mut temp_matrix);
        i += 1;
        m = 1 << i;
    }
    res_matrix[0][1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
