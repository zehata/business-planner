use uuid::Uuid;

pub fn print_list(list: Vec<(&Uuid, &str)>) {
    let longest_name_length = list.iter().fold(0_usize, |longest_name_length, (_, name)| {
        let name_length = name.len();
        if name_length > longest_name_length {
            return name_length
        };
        longest_name_length
    });
    list.into_iter().for_each(|(id, name)| {
        println!("{:longest_name_length$} {}", name, id);
    });
}