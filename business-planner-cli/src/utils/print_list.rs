pub fn print_list(list: Vec<&str>) {
    list.iter().for_each(|string| {
        println!("{}", string);
    });
}