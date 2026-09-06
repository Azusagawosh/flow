use git2::{Repository, RemoteCallbacks, PushOptions, Cred, StatusOptions, Status, FetchOptions};
use git2::build::RepoBuilder;
use std::path::Path;
pub fn init(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = Path::new(path);
    if !repo_path.exists() {
        std::fs::create_dir_all(repo_path)?;
    }
    match Repository::init(repo_path) {
        Ok(_) => {
            println!("Git репозиторий успешно инициализирован в '{}'", path);
            Ok(())
        }
        Err(e) => {
            println!("Ошибка при инициализации Git репозитория: {}", e);
            Err(Box::new(e))
        }
    }
}
pub fn add_and_commit(path: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = Repository::open(path)?;
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None
        )?;
        index.write()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let signature = repo.signature()?;
        let parent_commit = match repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None,
        };
        let commit_id = match parent_commit {
            Some(parent) => repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent])?,
            None => repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?,
        };
        println!("Изменения успешно зафиксированы с коммитом: {}", commit_id);
        Ok(())
}

pub fn add_all(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    println!("Все изменения успешно добавлены в индекс.");
    Ok(())
}

pub fn commit(path: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let signature = repo.signature()?;
    let parent_commit = match repo.head() {
        Ok(head) => Some(head.peel_to_commit()?),
        Err(_) => None,
    };
    let commit_id = match parent_commit {
        Some(parent) => repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent])?,
        None => repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?,
    };
    println!("Изменения успешно зафиксированы с коммитом: {}", commit_id);
    Ok(())
}

pub fn push(path: &str, remote_name: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut remote = repo.find_remote(remote_name)?;
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::userpass_plaintext("git", "DUMMY_GITHUB_TOKEN")
    });
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);
    remote.push(&[&format!("refs/heads/{}:refs/heads/{}", branch, branch)], Some(&mut push_options))?;
    println!("Изменения успешно отправлены на удаленный репозиторий '{}', ветка '{}'", remote_name, branch);
    Ok(())
}

pub fn status(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    let mut files = Vec::new();
    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("?");
        let status = entry.status();
        let prefix = match status {
            s if s.contains(Status::WT_NEW) => "[A]",
            s if s.contains(Status::WT_MODIFIED) => "[M]",
            s if s.contains(Status::WT_DELETED) => "[D]",
            s if s.contains(Status::INDEX_NEW) => "[??]",
            s if s.contains(Status::INDEX_MODIFIED) => "[M]",
            s if s.contains(Status::INDEX_DELETED) => "[D]",
            _ => "[]",
        };
        files.push(format!("{} {}", prefix, path));
}
Ok(())
}

pub fn clone_repo(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::userpass_plaintext("git", "DUMMY_GITHUB_TOKEN")
    });
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    match builder.clone(url, Path::new(path)) {
        Ok(_) => {
            println!("Репозиторий успешно клонирован из '{}' в '{}'", url, path);
            Ok(())
        }
        Err(e) => {
            println!("Ошибка при клонировании репозитория: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn remote_add(path: &str, name: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    repo.remote(name, url)?;
    println!("Удаленный репозиторий '{}' успешно добавлен с URL '{}'", name, url);
    Ok(())
}