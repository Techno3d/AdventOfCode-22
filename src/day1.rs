pub fn day1(elf_info: String) {
    /*
    let mut elf_num = 0;
    let mut max_elf_cal = 0;
    for elem in elf_info.split('\n') {
        if elem == "" {
            elf_num += 1;
            continue;
        }
        max_elf_cal += elem.parse::<i32>().unwrap();
    }
    */
   let mut elems: Vec<&str> = elf_info.split("\n\n").collect();
   elems.pop();
   let mut elf_nums = Vec::new();
   elf_nums.resize(elems.len()+1, 0);

    let mut most_cal_elf = 0;
    let mut second = 0;
    let mut third = 0;

    for (i, elem) in elems.iter().enumerate() {
        for num in elem.split('\n') {
            elf_nums[i] += num.parse::<i64>().unwrap();
        }
        if i != 0 {
            if elf_nums[i] > elf_nums[most_cal_elf] {
                third = second;
                second = most_cal_elf;
                most_cal_elf = i;
            } else if elf_nums[i] > elf_nums[second] {
                third = second;
                second = i
            } else if elf_nums[i] > elf_nums[third] {
                third = i;
            }
        }
    }

    println!("Elf number {}, has the most cals of {}", most_cal_elf + 1, elf_nums[most_cal_elf]);
    println!("Elf number {}, has the second most cals of {}", second + 1, elf_nums[second]);
    println!("Elf number {}, has the third most cals of {}", third + 1, elf_nums[third]);
    println!("The total of the top 3 is {}", elf_nums[most_cal_elf] + elf_nums[second] + elf_nums[third]);
}
