use git2::{Repository, RemoteCallbacks, Cred, StatusOptions, Status, FetchOptions};
use git2::build::RepoBuilder;
use std::path::Path;
use chrono::{DateTime, Utc};

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
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
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
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
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

    for entry in statuses.iter() {
        let file_path = entry.path().unwrap_or("?");
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
        println!("{} {}", prefix, file_path);
    }
    Ok(())
}

pub fn clone_repo(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
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

pub fn remote_remove(path: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    repo.remote_delete(name)?;
    println!("Удаленный репозиторий '{}' успешно удален", name);
    Ok(())
}

pub fn remote_list(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let remotes = repo.remotes()?;
    println!("Список удаленных репозиториев:");
    for remote_name in remotes.iter() {
        if let Some(name) = remote_name {
        if let Ok(remote) = repo.find_remote(name) {
           let url = remote.url().unwrap_or("?");
            println!("- {}: {}", name, url);
        }
    }
}
    Ok(())
}

pub fn fetch(path: &str, remote_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut remote = repo.find_remote(remote_name)?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("git", "DUMMY_GITHUB_TOKEN")
    });
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options), None)?;
    println!("Изменения успешно получены с удаленного репозитория '{}'", remote_name);
    Ok(())
}

pub fn pull(path: &str, remote_name: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut remote = repo.find_remote(remote_name)?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("git", "DUMMY_GITHUB_TOKEN")
    });
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    remote.fetch(&[branch], Some(&mut fetch_options), None)?;
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", branch);
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        println!("Изменения успешно объединены с удаленного репозитория '{}', ветка '{}'", remote_name, branch);
    } else {
        println!("Нет изменений для объединения с удаленного репозитория '{}', ветка '{}'", remote_name, branch);
    }
    Ok(())
}

pub fn get_current_branch(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let head = repo.head()?;
    if head.is_branch() {
    if let Some(branch_name) = head.shorthand() {
        return Ok(branch_name.to_string());
        }   
    }
    Err("Не удалось определить текущую ветку".into())
}

pub fn checkout_branch(path: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let (object, reference) = repo.revparse_ext(branch)?;
    repo.checkout_tree(&object, None)?;
    match reference {
        Some(r) => repo.set_head(r.name().unwrap())?,
        None => repo.set_head_detached(object.id())?,
    }
    println!("Успешно переключено на ветку '{}'", branch);
    Ok(())
}

pub fn create_branch(path: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let head = repo.head()?;
    let target_commit = head.peel_to_commit()?;
    repo.branch(branch, &target_commit, false)?;
    println!("Ветка '{}' успешно создана", branch);
    Ok(())
}

pub fn delete_branch(path: &str, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut branch_ref = repo.find_branch(branch, git2::BranchType::Local)?;
    branch_ref.delete()?;
    println!("Ветка '{}' успешно удалена", branch);
    Ok(())
}

pub fn list_branches(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let branches = repo.branches(Some(git2::BranchType::Local))?;
    println!("Список локальных веток:");
    for branch in branches {
        let (branch, _) = branch?;
        if let Ok(Some(name)) = branch.name() {
            println!("- {}", name);
        }
    }
    Ok(())
}

pub fn get_remote_url(path: &str, remote_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let remote = repo.find_remote(remote_name)?;
    match remote.url() {
        Some(url) => Ok(url.to_string()),
        None => Err("Удаленный репозиторий не имеет URL".into()),
    }
}

pub fn set_remote_url(path: &str, remote_name: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    repo.remote_delete(remote_name)?;
    repo.remote(remote_name, url)?;
    println!("URL удаленного репозитория '{}' успешно изменен на '{}'", remote_name, url);
    Ok(())
}

pub fn get_commit_history(path: &str, max_count: usize) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    println!("История коммитов (последние {}):", max_count);
    for (i, commit_id) in revwalk.enumerate().take(max_count) {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;
        let summary = commit.summary().unwrap_or("Без описания");
        println!("{}: {} - {}", i + 1, commit.id(), summary);
    }
    Ok(())
}

pub fn get_commit_details(path: &str, commit_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let oid = git2::Oid::from_str(commit_hash)?;
    let commit = repo.find_commit(oid)?;

    let timestamp = commit.time().seconds();
    let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap_or_default();

    println!("Детали коммита {}:", commit.id());
    println!("Автор: {} <{}>",
        commit.author().name().unwrap_or("Unknown"),
        commit.author().email().unwrap_or("Unknown"));
    println!("Дата: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
    println!("Сообщение: {}", commit.message().unwrap_or("Без описания"));
    Ok(())
}

pub fn get_diff(path: &str, commit_hash1: &str, commit_hash2: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let oid1 = git2::Oid::from_str(commit_hash1)?;
    let oid2 = git2::Oid::from_str(commit_hash2)?;
    let commit1 = repo.find_commit(oid1)?;
    let commit2 = repo.find_commit(oid2)?;
    let tree1 = commit1.tree()?;
    let tree2 = commit2.tree()?;
    let diff = repo.diff_tree_to_tree(Some(&tree1), Some(&tree2), None)?;
    println!("Различия между коммитами {} и {}:", commit_hash1, commit_hash2);
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        print!("{}", std::str::from_utf8(line.content()).unwrap_or(""));
        true
    })?;
    Ok(())
}

pub fn get_file_content_at_commit(path: &str, commit_hash: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let oid = git2::Oid::from_str(commit_hash)?;
    let commit = repo.find_commit(oid)?;
    let tree = commit.tree()?;
    let entry = tree.get_path(Path::new(file_path))?;
    let blob = repo.find_blob(entry.id())?;
    let content = std::str::from_utf8(blob.content())?;
    println!("Содержимое файла '{}' на коммите {}:\n{}", file_path, commit_hash, content);
    Ok(())
}

pub fn get_commit_count(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    let count = revwalk.count();
    Ok(count)
}

pub fn get_commit_hashes(path: &str, max_count: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    let mut hashes = Vec::new();
    for commit_id in revwalk.take(max_count) {
        let commit_id = commit_id?;
        hashes.push(commit_id.to_string());
    }
    Ok(hashes)
}

pub fn get_commit_messages(path: &str, max_count: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    let mut messages = Vec::new();
    for commit_id in revwalk.take(max_count) {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;
        let summary = commit.summary().unwrap_or("Без описания");
        messages.push(summary.to_string());
    }
    Ok(messages)
}