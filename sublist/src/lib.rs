#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let fl_len = first_list.len();
    let sl_len = second_list.len();
    if fl_len == 0 && sl_len == 0 {
        return Comparison::Equal;
    }
    if fl_len == 0 {
        return Comparison::Sublist;
    }
    if sl_len == 0 {
        return Comparison::Superlist;
    }
    if fl_len >= sl_len {
        return sublist_p(first_list, second_list)
    }
    let blueberry = sublist_p(second_list, first_list);
    if blueberry == Comparison::Superlist {
        // ^^ need to swap the super/sub list enum because we passed in the second list as the bigger list
        return Comparison::Sublist;
    }
    blueberry
}

fn sublist_p(bigger: &[i32], smaller: &[i32]) -> Comparison {
    let mut sml_idx = 0;
    let mut sml_val = smaller[sml_idx];
    let sml_len = smaller.len();
    let big_len = bigger.len();

    let mut i = 0;
    
    while i < big_len {        
        let big_val = bigger[i];
        
        if sml_val == big_val {
            sml_idx += 1;
            if sml_len == sml_idx {
                // the lists are equal, or the small list is contained inside the big list
                if big_len > sml_len {
                    return Comparison::Superlist;
                }
                return Comparison::Equal;
            }
        }
        else {
            i -= sml_idx;
            sml_idx = 0;
        }
        
        sml_val = smaller[sml_idx];
        i += 1;
    }
    
    Comparison::Unequal
}