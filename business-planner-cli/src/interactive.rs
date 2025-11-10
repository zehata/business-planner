use inquire::{InquireError, Select};

pub fn interactive() {
    let mut user_requested_exit = false;

    while !user_requested_exit {
        let commands: Vec<&str> = vec![
            "Save",
            "Exit",
        ];

        let ans: Result<&str, InquireError> = Select::new("Select", commands).prompt();

        match ans {
            Ok(command) => {
                match command {
                    "Save" => {},
                    "Exit" => { user_requested_exit = true },
                    _ => println!("There was an error, please try again"),
                }
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
}