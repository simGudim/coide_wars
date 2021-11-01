pub fn pos_average(s: &str)  {
    let str_list: Vec<&str>  = s.split(',').collect();
    let str_length: usize = str_list[0].len();
    let list_length: usize = str_list.len();
    let mut result_left: usize = vec![];
    let mut result_right: usize = vec![];

    println!("{} {}", str_length, list_length);


}