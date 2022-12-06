use std::collections::VecDeque;

pub fn day5(stacks: &String) {
    let split_data: Vec<&str> = stacks.splitn(2, "\n\n").collect();
    let (diagram_unaltered, moves) = (split_data.get(0).unwrap_or(&"").to_owned(), 
                            split_data.get(1).unwrap_or(&"").to_owned());
    let diagram = diagram_unaltered.replace("    ", "]");
    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in diagram.split('\n') {
        rows.push(line.split("]").map(|x| x.trim_matches(|y| y == ' ' || y == '[' || y == '\n')).collect());
    }

    let mut stacks: VecDeque<VecDeque<&str>> = VecDeque::new();
    stacks.resize(9, VecDeque::new());
    for row in rows {
        for (i, elem) in row.iter().enumerate() {
            if elem == &"" || elem.contains('1') {
                continue;
            }

            stacks.get_mut(i).expect("Out of bounds").push_back(elem);
        }
    }
    let empty: Vec<&str> = Vec::new();
    stacks.retain(|x| x != &empty);
    
    // Code part
    let lines: Vec<&str> = moves.split('\n').collect();
    for line in lines {
        if line == "" {
            continue;
        }
        let remove_from: Vec<&str> = line.split("from").collect();
        let (first, remove_to) = (remove_from.get(0).unwrap().trim_matches(|x| x == 'm' || x == 'o' || x == 'v' || x == 'e' || x == ' ').parse::<usize>().unwrap(), //Is this sus? yes
                remove_from.get(1).unwrap().trim().split("to").collect::<Vec<&str>>());
        let (second, third) = (remove_to.get(0).unwrap().trim().parse::<usize>().unwrap(), remove_to.get(1).unwrap().trim().parse::<usize>().unwrap());


        for _ in 0..first {
            let mut held = "";
            {
                let stack_from = stacks.get_mut(second - 1).unwrap();
                held = stack_from.pop_front().unwrap();
            }

            {
                let stack_to = stacks.get_mut(third - 1).unwrap();
                stack_to.push_front(held);
            }
        }
    }
    
    let mut top_level = String::new();
    for stack in &stacks {
        top_level += stack.get(0).expect("");
    }
    
    println!("{:?}\n{}", stacks, top_level);
}

pub fn day5_part2(stacks: &String) {
    let split_data: Vec<&str> = stacks.splitn(2, "\n\n").collect();
    let (diagram_unaltered, moves) = (split_data.get(0).unwrap_or(&"").to_owned(), 
                            split_data.get(1).unwrap_or(&"").to_owned());
    let diagram = diagram_unaltered.replace("    ", "]");
    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in diagram.split('\n') {
        rows.push(line.split("]").map(|x| x.trim_matches(|y| y == ' ' || y == '[' || y == '\n')).collect());
    }

    let mut stacks: VecDeque<VecDeque<&str>> = VecDeque::new();
    stacks.resize(9, VecDeque::new());
    for row in rows {
        for (i, elem) in row.iter().enumerate() {
            if elem == &"" || elem.contains('1') {
                continue;
            }

            stacks.get_mut(i).expect("Out of bounds").push_back(elem);
        }
    }
    let empty: Vec<&str> = Vec::new();
    stacks.retain(|x| x != &empty);
    //Code part 2
    let lines: Vec<&str> = moves.split('\n').collect();
    for line in lines {
        if line == "" {
            continue;
        }
        let remove_from: Vec<&str> = line.split("from").collect();
        let (first, remove_to) = (remove_from.get(0).unwrap().trim_matches(|x| x == 'm' || x == 'o' || x == 'v' || x == 'e' || x == ' ').parse::<usize>().unwrap(), //Is this sus? yes
                remove_from.get(1).unwrap().trim().split("to").collect::<Vec<&str>>());
        let (second, third) = (remove_to.get(0).unwrap().trim().parse::<usize>().unwrap(), remove_to.get(1).unwrap().trim().parse::<usize>().unwrap());

        let mut held: Vec<&str> = Vec::new();
        for _ in 0..first {
            {
                let stack_from = stacks.get_mut(second - 1).unwrap();
                held.push(stack_from.pop_front().unwrap());
            }

        }
        {
            let stack_to = stacks.get_mut(third - 1).unwrap();
            for crates in held.iter().rev() {
                stack_to.push_front(crates);
            }
        }
    }

    let mut top_level = String::new();
    for stack in &stacks {
        top_level += stack.get(0).expect("");
    }

    println!("{:?}\n{}", stacks, top_level);
}
