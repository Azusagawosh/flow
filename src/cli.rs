use std::env;
use crate::projects;
use crate::terminal;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Использование: flow <команда> [аргументы]");
        println!("Команды: new <имя>, list, delete <имя>, note <имя>");
        return;
    }

    let command = &args[1];

    if command == "new" {
        if args.len() < 3 {
            println!("Укажите имя проекта: flow new <имя>");
            return;
        }
        projects::create_project(args[2].clone());
    } else if command == "list" {
        projects::list_projects();
    } else if command == "delete" {
        if args.len() < 3 {
            println!("Укажите имя проекта: flow delete <имя>");
            return;
        }
        projects::remove_project(args[2].clone());
    } else if command == "note" {
        if args.len() < 3 {
            println!("Укажите имя заметки: flow note <имя>");
            return;
        }
        terminal::on_start(args[2].clone());
    } else {
        println!("Неизвестная команда: {}", command);
        println!("Доступные: new, list, delete, note");
    }
}

