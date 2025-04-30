pub fn egg_count(display_value: u32) -> usize {
    let mut eggs = 0;
    let mut num = display_value;
    let mut power: u32 = 31;
    let base: u32 = 2;

    while num > 0 {
       let pow = base.pow(power);
       if num >= pow {
           eggs += 1;
           num -= pow;
        }
       if power > 0 {
           power -= 1;
        }
       else {
           break;
        }
    }
    
    return eggs;
}

