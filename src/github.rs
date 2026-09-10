use octocrab::Octocrab;
use octocrab::models::IssueState;
use crate::config::Config;

fn get_client() -> Result<Octocrab, Box<dyn std::error::Error>> {
    let config = Config::load();
    if config.github.token.is_empty() {
        return Err("GitHub токен не настроен. Выполни: flow config set-token <токен>".into());
    }
    Ok(Octocrab::builder()
        .personal_token(config.github.token)
        .build()?)
}

async fn get_owner(client: &Octocrab) -> Result<String, Box<dyn std::error::Error>> {
    Ok(client.current().user().await?.login)
}

pub async fn create_repo(repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    let client = get_client()?;

    let create_params = serde_json::json!({
        "name": repo_name,
        "private": config.github.default_private,
    });

    let repo = client
        .post::<_, serde_json::Value>("user/repos", Some(&create_params))
        .await?;

    println!("Репозиторий '{}' успешно создан на GitHub!", repo["name"].as_str().unwrap_or("?"));
    println!("URL: {}", repo["html_url"].as_str().unwrap_or("?"));
    Ok(())
}

pub async fn get_user() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let user = client.current().user().await?;
    println!("Авторизован как: {} (@{})", user.name.unwrap_or_else(|| "Unknown".into()), user.login);
    Ok(())
}

pub async fn list_repos() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let repos = client
        .get::<Vec<serde_json::Value>, _, ()>(
            &format!("/users/{}/repos?per_page=100", owner),
            None,
        )
        .await?;
    println!("Список репозиториев:");
    for repo in repos {
        println!(
            "- {} (URL: {})",
            repo["name"].as_str().unwrap_or("?"),
            repo["html_url"].as_str().unwrap_or("?")
        );
    }
    Ok(())
}

pub async fn delete_repo(repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    client
        .delete::<(), _, ()>(&format!("/repos/{}/{}", owner, repo_name), None::<&()>)
        .await?;
    println!("Репозиторий '{}' успешно удален с GitHub!", repo_name);
    Ok(())
}

pub async fn create_issue(repo_name: &str, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let issue = client.issues(&owner, repo_name).create(title).body(body).send().await?;
    println!("Issue '{}' успешно создан в репозитории '{}'", issue.title, repo_name);
    println!("URL: {}", issue.html_url);
    Ok(())
}

pub async fn list_issues(repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let issues = client.issues(&owner, repo_name).list().send().await?;
    println!("Список issues в репозитории '{}':", repo_name);
    for issue in issues.items {
        println!("- #{}: {} (URL: {})", issue.number, issue.title, issue.html_url);
    }
    Ok(())
}

pub async fn close_issue(repo_name: &str, issue_number: u64) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    client
        .issues(&owner, repo_name)
        .update(issue_number)
        .state(IssueState::Closed)
        .send()
        .await?;
    println!("Issue #{} успешно закрыт в репозитории '{}'", issue_number, repo_name);
    Ok(())
}

pub async fn get_issue(repo_name: &str, issue_number: u64) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let issue = client.issues(&owner, repo_name).get(issue_number).await?;
    println!("Issue #{}: {}", issue.number, issue.title);
    println!("Статус: {:?}", issue.state);
    println!("URL: {}", issue.html_url);
    println!("Описание:\n{}", issue.body.unwrap_or_else(|| "Нет описания".into()));
    Ok(())
}

pub async fn get_repo_url(repo_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let repo = client.repos(&owner, repo_name).get().await?;
    Ok(repo.html_url.map(|u| u.to_string()).unwrap_or_default())
}

pub async fn get_issue_url(repo_name: &str, issue_number: u64) -> Result<String, Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let issue = client.issues(&owner, repo_name).get(issue_number).await?;
    Ok(issue.html_url.to_string())
}

pub async fn get_repo_details(repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let repo = client.repos(&owner, repo_name).get().await?;
    println!("Репозиторий: {}", repo.name);
    println!("Описание: {}", repo.description.unwrap_or_else(|| "Нет описания".into()));
    println!("URL: {:?}", repo.html_url);
    println!("Приватный: {}", repo.private.unwrap_or(false));
    println!("Создан: {:?}", repo.created_at);
    println!("Обновлен: {:?}", repo.updated_at);
    println!("Ветка по умолчанию: {:?}", repo.default_branch);
    Ok(())
}

pub async fn list_user_repos() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client()?;
    let owner = get_owner(&client).await?;
    let repos = client
        .get::<Vec<serde_json::Value>, _, ()>(
            &format!("/users/{}/repos?per_page=100", owner),
            None,
        )
        .await?;
    println!("Список репозиториев пользователя '{}':", owner);
    for repo in repos {
        println!(
            "- {} (URL: {})",
            repo["name"].as_str().unwrap_or("?"),
            repo["html_url"].as_str().unwrap_or("?")
        );
    }
    Ok(())
}