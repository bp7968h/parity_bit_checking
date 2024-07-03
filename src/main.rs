fn main(){
    let abc = b"abc";
    println!("input: {:?} output: {:08x}", abc, parity_bit(abc));
    
    println!();
    
    let abcd = b"abcd";
    println!("input: {:?} output: {:08x}",abcd, parity_bit(abcd))
}


//The parity_bit() function takes an arbitrary stream of bytes and returns a u8, indicating whether the count of the input’s bits was even or odd.
fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;
    
    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }
    println!("n_ones: {}", n_ones);
    (n_ones % 2 == 0) as u8
}
