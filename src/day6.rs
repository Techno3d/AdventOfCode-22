use std::collections::HashSet;

pub fn day6(stream: &String) {
    let char_stream: Vec<char> = stream.chars().collect();
    let mut char_buf: HashSet<char> = HashSet::new();
    let mut start_pos = 0;
    
    'outer: for i in 0..char_stream.len() {
        char_buf.clear();
        for j in 0..4 {
            if !char_buf.insert(match char_stream.get(i + j) {
                Some(x) => *x,
                None => continue,
            }) {
                continue 'outer;
            }
        }

        if char_buf.len() == 4 {
            start_pos = i;
            break;
        }
    }
    println!("Chars Processed: {}\n{} {:?}", start_pos, start_pos + 4, char_buf);

    //Part 2
    let char_stream: Vec<char> = stream.chars().collect();
    start_pos = 0;
    'outer: for i in 0..char_stream.len() {
        char_buf.clear();
        for j in 0..14 {
            if !char_buf.insert(match char_stream.get(i + j) {
                Some(x) => *x,
                None => continue,
            }) {
                continue 'outer;
            }
        }
        println!("{:?}", char_buf);

        if char_buf.len() == 14 {
            start_pos = i;
            break;
        }
    }
    println!("\nPart2:\nChars Processed: {}\n{} {:?}", start_pos, start_pos + 14, char_buf);
}
