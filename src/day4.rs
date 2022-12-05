pub fn day4(pair_list: &String) {
    let mut total_overlap = 0;
    let mut total_any_overlap = 0;
    for pair in pair_list.split('\n') {
        if pair == "" {
            continue;
        }
        let pair_range: Vec<(i32, i32)> = pair.split(',').map(|x| {
            let range = x.split_at(x.chars().position(|x| x == '-').unwrap() + 1);
            return (range.0.trim_end_matches('-').parse::<i32>().unwrap_or_default(), range.1.trim_end_matches('-').parse::<i32>().unwrap_or_default())
        } ).collect();

        let range1 = pair_range.get(0).unwrap();
        let range2 = pair_range.get(1).unwrap();
        if range1.0 >= range2.0 && range1.1 <= range2.1 {
            total_overlap += 1;
        } else if range2.0 >= range1.0 && range2.1 <= range1.1 {
            total_overlap += 1;
        }

        if range1.0 >= range2.0 && range1.0 <= range2.1 || range1.1 >= range2.0 && range1.1 <= range2.1 {
            total_any_overlap += 1;
        } else if range2.0 >= range1.0 && range2.0 <= range1.1 || range2.1 >= range1.0 && range2.1 <= range1.1 {
            total_any_overlap += 1;
        }
    }
    println!("{}", total_overlap);
    println!("{}", total_any_overlap);
}
