use octocrab::Octocrab;
use crate::config::Config;

pub async fn create_repo(repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    if config.github.token.is_empty() {
        return Err("GitHub токен не настроен. Выполни: flow config set-token <токен>".into());
    }
    let octocrab = Octocrab::builder()
        .personal_token(config.github.token)
        .build()?;

    // Используем serde_json вместо CreateRepo
    let create_params = serde_json::json!({
        "name": repo_name,
        "private": config.github.default_private,
    });

    let repo = octocrab
        .post::<_, serde_json::Value>("user/repos", Some(&create_params))
        .await?;

    println!("Репозиторий '{}' успешно создан на GitHub!", repo["name"].as_str().unwrap_or("?"));
    println!("URL: {}", repo["html_url"].as_str().unwrap_or("?"));

    Ok(())
}

pub async fn get_user() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();

    if config.github.token.is_empty() {
        return Err("GitHub токен не настроен".into());
    }
    let octocrab = Octocrab::builder()
        .personal_token(config.github.token)
        .build()?;
    let user = octocrab.current().user().await?;
    println!("Авторизован как: {} (@{})", user.name.unwrap_or_else(|| "Unknown".into()), user.login);
    Ok(())
}