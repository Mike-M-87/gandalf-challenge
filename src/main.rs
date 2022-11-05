//euclidean_gcd uses the Extended Euclidean algorithms that takes two integers ‘a’ and ‘b’
// then finds their gcd, and also find ‘x’ and ‘y’ such that
// ax + by = gcd(a, b)
fn euclidean_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a != 0 {
        let modulus = b % a;

        let (gcd, coefficient_a, coefficient_b) = euclidean_gcd(modulus, a);

        let quotient = b / a;

        let new_a = coefficient_b - (quotient * coefficient_a);

        return (gcd, new_a, coefficient_a);
    }

    return (b, 0, 1);
}
// modular_multiplicative_inverse returns the
// Modular multiplicative inverse of an interger A  such that
// A X ≅ 1 (mod M)
fn modular_multiplicative_inverse(a: isize, m: isize) -> Option<isize> {
    let (gcd, coefficient_a, _) = euclidean_gcd(a, m);

    if gcd != 1 {
        return None;
    }

    return Some((coefficient_a % m + m) % m);
}

use std::io;

fn main() {
    //the user inputs two values which are passed as parameters into the 
    //modular multiplicative inverse function
    loop {
        println!("Enter two values to compute Modular Multiplicative Inverse for e.g 10 17");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let inputs: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let a = inputs[0];
        let m = inputs[1];

        let res = modular_multiplicative_inverse(a, m);
        //if the result returned is None  the modulo inverse cannot be computed
        match res {
            Some(modulo_inv) => {
                println!("Modular Multiplicative inverse of {a} and {m} = {modulo_inv} \n");
            }
            None => {
                println!("Modular Multiplicative inverse cannot be computed\n");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modular_multiplicative_inverse;
    #[test]
    fn it_works() {
        assert!(
            modular_multiplicative_inverse(15, 5).is_none(),
            "a is greater than mod"
        );
        assert_eq!(modular_multiplicative_inverse(10, 17).unwrap(), 12);
        assert_eq!(modular_multiplicative_inverse(3, 11).unwrap(), 4);
    }
}
