fn gcd(mut n: u64, mut m: u64) -> u64 {

    assert!(n != 0 && m != 0);
    
    while m != 0 {
        
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        
        m = m % n;
    }
    
    n

}

fn main() {
    let num1 = 30;
    let num2 = 40;
    let result = gcd(num1, num2);
    println!("{} ve {}'nin EBOB'u: {}", num1, num2, result);
}
