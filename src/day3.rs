use std::collections::HashSet;

pub fn day3(rucksacks: &String) {
    let data = rucksacks.split('\n').map(|x| x.split_at(x.len()/2) );
    let mut chars_list: Vec<char> = ('a'..='z').collect();
    chars_list.append(&mut ('A'..='Z').collect());
    let mut vals: Vec<usize> = Vec::new();

    for rucksack in data {
        if rucksack.0 == "" {
            continue
        }

        let set_a: HashSet<char> = rucksack.0.chars().collect::<HashSet<char>>();
        let set_b: HashSet<char> = rucksack.1.chars().collect::<HashSet<char>>();
        let union_elems = set_a.intersection(&set_b);

        for item in union_elems {
            vals.push(chars_list.iter().position(|&x| &x == item).unwrap_or(0) + 1);
        }
    }
    println!("{}", vals.iter().sum::<usize>());
}

pub fn day3_part2(rucksacks: &String) {
    let data: Vec<&str> = rucksacks.split('\n').collect();
    let mut chars_list: Vec<char> = ('a'..='z').collect();
    chars_list.append(&mut ('A'..='Z').collect());
    let mut vals: Vec<usize> = Vec::new();

    for group in data.chunks(3) {
        if group.len() != 3 {
            continue;
        }
        let a = group[0].chars().collect::<HashSet<char>>();
        let b = group[1].chars().collect::<HashSet<char>>();
        let c = group[2].chars().collect::<HashSet<char>>();
        let shared = a.intersection(&b)
            .collect::<HashSet<&char>>();
        let mut shared2: HashSet<char> = HashSet::new();
        for elem in shared {
            shared2.insert(elem.to_owned());
        }

        let shared_final = shared2.intersection(&c);
        for elem in shared_final {
            vals.push(chars_list.iter().position(|&x| &x == elem).unwrap_or(0) + 1);
        }
    }

    println!("{}", vals.iter().sum::<usize>());
}
