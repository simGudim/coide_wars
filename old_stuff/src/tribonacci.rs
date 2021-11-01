pub fn clean_string(s: &str) {
    let mut s_list: Vec<char> = s.chars().collect::<Vec<char>>();
    let mut remove_index: Vec<usize> = vec![];

    for (i, x) in s_list.iter().enumerate() {
        if s_list[i] == '#' {
            remove_index.push(i);
        }
    }

    remove_index.dedup();

    // for i in remove_index.iter().rev() {
    //     s_list.remove(*i);
    // }
    println!("{:?}", remove_index);
    println!("{:?}", s_list);
    // s_list.join
}