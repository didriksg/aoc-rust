use std::cell::RefCell;
use std::rc::Rc;

struct Folder {
    pub size: Option<usize>,
    pub parent: Option<Rc<RefCell<Folder>>>,
}

impl Folder {
    pub fn new() -> Folder {
        Folder {
            size: Some(0),
            parent: None,
        }
    }
}

fn add_to_size(folder: &Rc<RefCell<Folder>>, value: usize) {
    let mut folder = folder.borrow_mut();
    folder.size = Some(value);
}

fn process_command(command: &str, current_folder: &Rc<RefCell<Folder>>, all_folders: &mut Vec<Rc<RefCell<Folder>>>) -> Rc<RefCell<Folder>> {
    return if command.contains("$ ls") {
        Rc::clone(current_folder)
    } else if command.contains("$ cd ..") {
        let current_clone = Rc::clone(current_folder);
        let parent = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        let new_size = current_folder.borrow().size.unwrap() + parent.borrow().size.unwrap();
        add_to_size(&parent, new_size);

        parent
    } else {
        let child = Rc::new(RefCell::new(Folder::new()));
        {
            let mut mut_child = child.borrow_mut();
            mut_child.parent = Some(Rc::clone(current_folder));
        }

        all_folders.push(child.clone());

        child
    };
}


fn add_to_folder(command: &str, current_folder: &Rc<RefCell<Folder>>) {
    if command.contains("dir") { return; }

    let (size_str, _) = command.split_once(" ").unwrap();
    let new_value = current_folder.borrow().size.unwrap() + size_str.parse::<usize>().unwrap();

    add_to_size(&current_folder, new_value);
}


fn main() {
    let root = Rc::new(RefCell::new(Folder::new()));
    let mut all_folders: Vec<Rc<RefCell<Folder>>> = Vec::new();
    let mut current = Rc::clone(&root);

    all_folders.push(root.clone());

    let commands = include_str!("../input.txt").lines().skip(1);

    for command in commands {
        if command.contains("$") {
            current = process_command(command, &current, &mut all_folders);
        } else {
            add_to_folder(command, &current);
        }
    }

    while !current.borrow().parent.is_none() {
        let current_clone = Rc::clone(&current);
        let parent = Rc::clone(&current_clone.borrow().parent.as_ref().unwrap());
        let new_value = parent.borrow().size.unwrap() + current.borrow().size.unwrap();
        add_to_size(&parent, new_value);

        current = parent;
    }

    let part_1_answer: usize = all_folders
        .iter()
        .map(|c| c.borrow().size.unwrap())
        .filter(|c| *c <= 100000)
        .sum();

    const TOTAL_SPACE: usize = 70000000;
    const SPACE_NEEDED_FOR_UPDATE: usize = 30000000;
    let unused_space = TOTAL_SPACE - &root.borrow().size.unwrap();

    let part_2_answer = all_folders
        .iter()
        .map(|c| c.borrow().size.unwrap())
        .filter(|c| *c + unused_space >= SPACE_NEEDED_FOR_UPDATE)
        .min()
        .unwrap();

    println!("Part-1: {part_1_answer}");
    println!("Part-2: {part_2_answer}");
}
