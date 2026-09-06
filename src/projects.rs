use std::fs;
pub fn create_project(name: String) {
    let project_dir = format!("flow_store/projects/{}", name);
    if fs::create_dir_all(&project_dir).is_ok() {
        println!("Проект '{}' успешно создан!", name);
    } else {
        println!("Ошибка при создании проекта '{}'", name);
    }
}
pub fn remove_project(name: String) {
    let project_dir = format!("flow_store/projects/{}", name);
    if fs::remove_dir_all(project_dir).is_ok() {
        println!("Проект '{}' успешно удален!", name);
    }else {
        println!("Ошибка при удалении проекта '{}'", name);
    }
}
pub fn list_projects() {
    let projects_dir = "flow_store/projects";
    match fs::read_dir(projects_dir) {
        Ok(entries) => {
            println!("Список проектов:");
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            println!("- {}", entry.file_name().to_string_lossy());
                        }
                    }
                }
            }
        }
error => {
        println!("Ошибка при чтении директории проектов: {:?}", error);
}
    }
}