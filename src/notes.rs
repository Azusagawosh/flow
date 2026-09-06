use std::fs;
pub fn save_note(name: String, content: String) {
    let path = format!("flow_store/notes/{}.md", name);
    if fs::write(&path, content).is_ok() {
        println!("Заметка '{}' успешно сохранена!", name);
    } else {
        println!("Ошибка при сохранении заметки '{}'", name);
    }
}