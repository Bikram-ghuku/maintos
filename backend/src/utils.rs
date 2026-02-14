use std::{path::PathBuf, str::FromStr};

use anyhow::anyhow;
use git2::Repository;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::fs;

use crate::{auth::Auth, env::EnvVars, github};

pub(crate) type Res<T> = Result<T, anyhow::Error>;

#[derive(Deserialize, Serialize)]
/// All the information for a repository
pub struct Deployment {
    name: String,
    repo_url: String,
    repo_owner: String,
    repo_name: String,
}

/// Returns all information (repo, URL, name) of a deployment given its path
pub async fn get_deployment_info(
    env_vars: &EnvVars,
    auth: &Auth,
    project_name: &str,
) -> Res<Deployment> {
    let deployments_dir = &env_vars.deployments_dir;

    let repo_path = deployments_dir.join(project_name);
    let repo = Repository::open(repo_path)?;

    let repo_url = repo
        .find_remote("origin")?
        .url()
        .ok_or(anyhow!(
            "Error: Origin remote URL not found for repo {project_name}."
        ))?
        .to_string();
    let parsed_url = Url::from_str(&repo_url)?;

    let mut url_paths = parsed_url
        .path_segments()
        .ok_or(anyhow!("Error parsing repository remote URL."))?;
    let repo_owner = url_paths
        .next()
        .ok_or(anyhow!(
            "Error parsing repository remote URL: Repo owner not found."
        ))?
        .to_string();
    let repo_name = url_paths
        .next()
        .ok_or(anyhow!(
            "Error parsing repository remote URL: Repo name not found."
        ))?
        .to_string();

    if repo_owner == env_vars.gh_org_name {
        return Ok(Deployment {
            name: project_name.to_owned(),
            repo_url,
            repo_owner,
            repo_name,
        });
    }

    Err(anyhow!(
        "Error checking user {}'s access to project {}.",
        auth.username,
        project_name
    ))
}

pub async fn has_access(auth: &Auth, deployment: &Deployment) -> Res<bool> {
    let client = reqwest::Client::new();

    let collab_role = github::get_collaborator_role(
        &client,
        &auth.gh_access_token,
        &deployment.repo_owner,
        &deployment.repo_name,
        &auth.username,
    )
    .await?;

    // `None` means the user is not a collaborator
    if let Some(role) = collab_role.as_deref()
        && (role == "maintain" || role == "admin")
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Get a list of deployments
pub async fn get_deployments(env_vars: &EnvVars, auth: &Auth) -> Res<Vec<Deployment>> {
    let deployments_dir = &env_vars.deployments_dir;

    let mut deployments = Vec::new();

    let mut dir_iter = fs::read_dir(deployments_dir).await?;
    while let Some(path) = dir_iter.next_entry().await? {
        if path.file_type().await?.is_dir() {
            let project_name = path
                .file_name()
                .into_string()
                .map_err(|_| anyhow!("Invalid project name"))?;

            deployments.push(get_deployment_info(env_vars, auth, &project_name).await?);
        }
    }

    Ok(deployments)
}

#[derive(Deserialize, Serialize)]
/// Settings for a project
pub struct ProjectSettings {
    /// Subdirectory which is deployed (relative to the project root)
    pub deploy_dir: String,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        ProjectSettings {
            deploy_dir: String::from("."),
        }
    }
}

impl ProjectSettings {
    /// Get the project settings (stored in .maint on the top level of the project directory)
    pub async fn from_project(env_vars: &EnvVars, project_name: &str) -> Res<Self> {
        let maint_file_path = env_vars.deployments_dir.join(project_name).join(".maint");

        if let Ok(maint_file_contents) = fs::read_to_string(maint_file_path).await {
            Ok(Self {
                deploy_dir: maint_file_contents.trim().into(),
            })
        } else {
            Ok(Self::default())
        }
    }
}

/// Get the environment variables for a project
pub async fn get_env(
    env_vars: &EnvVars,
    auth: &Auth,
    project_name: &str,
) -> Res<Option<Map<String, Value>>> {
    let deployment = get_deployment_info(env_vars, auth, project_name).await?;
    let access = has_access(auth, &deployment).await?;

    if access {
        let project_settings = ProjectSettings::from_project(env_vars, project_name).await?;
        let env_file_path = PathBuf::from(&env_vars.deployments_dir)
            .join(project_name)
            .join(&project_settings.deploy_dir)
            .join(".env");

        if env_file_path.exists() {
            let parsed_env = dotenvy::from_path_iter(env_file_path)?
                .collect::<Result<Vec<(String, String)>, dotenvy::Error>>()?;

            Ok(Some(Map::from_iter(
                parsed_env
                    .into_iter()
                    .map(|(key, value)| (key, Value::String(value))),
            )))
        } else {
            Ok(None)
        }
    } else {
        Err(anyhow!("Error: No access to this project."))
    }
}
