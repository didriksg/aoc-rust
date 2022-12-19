struct Folder {
    parent: Option<Box<Folder>>,
    size: usize
}



fn process_command<'a>(command: &str, current_folder: &Folder) -> Folder {
    println!("{command}");
    let new_folder = Folder{parent: Option<Folder>(*current_folder.parent.unwrap()), size: *current_folder.size};

    return new_folder;
}


fn add_to_folder(command: &str, current_folder: &Folder) {
    println!("{command}");
}


fn main() {
    let folder_vector: Vec<Folder> = Vec::new();
    let mut current_folder = Folder{parent: None, size: 0};

    let commands = include_str!("../input.txt").lines();

    for command in commands {
        if command.contains("$") {
           current_folder = process_command(command, &current_folder)
        } else {
            add_to_folder(command, &current_folder);
        }
    }

    println!("Hello, world!");
}

