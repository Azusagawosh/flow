use std::fs;
pub fn init() {
    if fs::create_dir_all("flow_store/projects").is_ok() {
        println!("Папка projects успешно создана!");
    }else {
        println!("Ошибка при создании папки projects");
    }
    if fs::create_dir_all("flow_store/idea").is_ok() {
        println!("Папка idea успешно создана!");
    }else {
        println!("Ошибка при создании папки idea");
    }
    if fs::create_dir_all("flow_store/notes").is_ok() {
        println!("Папка notes успешно создана!");
    }else {
        println!("Ошибка при создании папки notes");
    }
}