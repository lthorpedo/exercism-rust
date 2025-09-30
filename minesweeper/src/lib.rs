pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    let max_rows = minefield.len();

    if max_rows < 1 {
        return ret;
    }
    
    let max_cols = minefield[0].len();
    
    let mut row = 0;
    let mut col = 0;

    while row < max_rows {
        let str = minefield[row].as_bytes();
        let mut ret_str: String = String::from("");
        while col < max_cols {
            if str[col] != b'*' {
                let blueberry = count_mines(minefield, row as i32, col as i32, max_rows, max_cols);
                println!("blueberry {}", blueberry);
                if blueberry > 0 {
                    ret_str.push_str(&blueberry.to_string());
                } else {
                    ret_str.push(' ');
                }
            } else {
                ret_str.push('*');
            }
            col += 1;
        }
        ret.push(ret_str);
        row += 1;
        col = 0;
    }

    ret
}

fn count_mines(minefield: &[&str], row: i32, col: i32, max_row: usize, max_col: usize) -> i32 {
    let mut cnt: i32 = 0;
    for i in -1..=1 {
        let temp_row: i32 = row + i;
        for j in -1..=1 {
            let temp_col: i32 = col + j;
            if is_star(minefield, temp_row, temp_col, max_row, max_col) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn is_star(minefield: &[&str], row: i32, col: i32, max_row: usize, max_col: usize) -> bool {
    if row > -1 && row < max_row as i32 && col > -1 && col < max_col as i32 {
        let character = minefield[row as usize].as_bytes()[col as usize];
        if character == b'*' {
            return true;
        }
    }
    false
}
