# Flow — CLI Project Manager

Инструмент управления проектами из терминала, написанный на Rust.

## Особенности

- **Управление проектами** — создание, список, удаление проектов
- **Заметки в Markdown** — интерактивный режим написания заметок прямо в терминале
- **Git интеграция** — init, add, commit, push, clone, status
- **GitHub API** — создание репозиториев, синхронизация (async)

## Технологии

- **Rust 2024 Edition**
- **git2** — локальные git операции
- **octocrab** — GitHub API (async)
- **tokio** — async runtime
- **crossterm** — терминал raw mode
- **serde + toml** — конфигурация

## Установка

```bash
git clone https://github.com/Azusagawosh/flow.git
cd flow
cargo build --release
./target/release/flow
```

## Использование

```bash
# Управление проектами
flow new my-project      # Создать проект
flow list                # Список проектов
flow delete my-project   # Удалить проект

# Заметки
flow note ideas          # Открыть заметку (raw mode)

# Git операции
flow git init            # Инициализировать репозиторий
flow git status          # Статус изменений
flow git commit "msg"    # Закоммитить

# GitHub интеграция
flow github create repo-name   # Создать репо на GitHub
flow github user               # Проверить авторизацию
```

## Структура проекта

```
flow_store/
├── projects/     # Локальные проекты
├── notes/        # Заметки в Markdown
├── idea/         # Идеи (в разработке)
└── config.toml   # Конфигурация (токен GitHub и др.)
```

## Статус

🚧 В активной разработке. Базовый функционал работает.

## Автор

**Azusagawosh** — [GitHub](https://github.com/Azusagawosh)

## Лицензия

MIT