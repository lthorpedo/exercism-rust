const MAX_BOTTLES: u32 = 10;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if start_bottles > MAX_BOTTLES || take_down > MAX_BOTTLES {
        return String::from("");
    }
    
    let mut bottles = start_bottles as i32;
    let mut iterations = take_down as i32;

    let mut ret = String::from("");

    while bottles > 0 && iterations > 0 {
        let bottles_str = if bottles == 1 {"bottle"} else {"bottles"};
        let num_str = uppercase_first_letter(&int_to_str(bottles));
    
        bottles -= 1;
        iterations -= 1;
        
        let bottles_left_str = if bottles == 1 {"bottle"} else {"bottles"};
        let num_bottles_left_str = int_to_str(bottles);
    
        let verse = format!("\n\n{num_str} green {bottles_str} hanging on the wall,\n\
                {num_str} green {bottles_str} hanging on the wall,\n\
                And if one green bottle should accidentally fall,\n\
                There'll be {num_bottles_left_str} green {bottles_left_str} hanging on the wall.");
        
        ret = ret + &verse;
    }

    ret
}

fn int_to_str(x: i32) -> String {
    match x {
        10 => String::from("ten"),
        9 => String::from("nine"),
        8 => String::from("eight"),
        7 => String::from("seven"),
        6 => String::from("six"),
        5 => String::from("five"),
        4 => String::from("four"),
        3 => String::from("three"),
        2 => String::from("two"),
        1 => String::from("one"),
        _ => String::from("no")
    }
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

