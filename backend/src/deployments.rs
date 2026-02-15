use std::{path::PathBuf, str::FromStr};

use anyhow::anyhow;
use git2::Repository;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::fs;

use crate::{auth::Auth, env::EnvVars, github, utils::Res};

#[derive(Deserialize, Serialize)]
/// All the information for a repository
pub struct Deployment {
    #[serde(skip_serializing)]
    deployment_path: PathBuf,

    pub deployment_dir: String,
    pub repo_url: String,
    pub repo_owner: String,
    pub repo_name: String,
}

impl Deployment {
    pub async fn from_deployment_dir(env_vars: &EnvVars, deployment_dir: &str) -> Res<Self> {
        let deployments_dir = &env_vars.deployments_dir;

        let deployment_path = deployments_dir.join(deployment_dir);
        let repo = Repository::open(&deployment_path)?;

        let repo_url = repo
            .find_remote("origin")?
            .url()
            .ok_or(anyhow!(
                "Error: Origin remote URL not found for repo {deployment_dir}."
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
            .to_string()
            .replace(".git", "");

        Ok(Self {
            deployment_path,
            deployment_dir: deployment_dir.to_owned(),
            repo_url,
            repo_owner,
            repo_name,
        })
    }

    pub async fn get_settings(&self) -> Res<DeploymentSettings> {
        DeploymentSettings::from_deployment(self).await
    }

    pub async fn has_access(&self, auth: &Auth) -> Res<bool> {
        let client = reqwest::Client::new();

        let collab_role = github::get_collaborator_role(
            &client,
            &auth.gh_access_token,
            &self.repo_owner,
            &self.repo_name,
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

    /// Get the environment variables for a project
    pub async fn get_env(&self) -> Res<Option<Map<String, Value>>> {
        let project_settings = self.get_settings().await?;
        let env_file_path = self
            .deployment_path
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
    }
}

#[derive(Deserialize, Serialize)]
/// Settings for a deployment, obtained from its `.maint` file
pub struct DeploymentSettings {
    /// Subdirectory which is deployed (relative to the project root)
    pub deploy_dir: String,
}

impl Default for DeploymentSettings {
    fn default() -> Self {
        DeploymentSettings {
            deploy_dir: String::from("."),
        }
    }
}

impl DeploymentSettings {
    /// Get the project settings (stored in .maint on the top level of the project directory)
    pub async fn from_deployment(deployment: &Deployment) -> Res<Self> {
        let maint_file_path = deployment.deployment_path.join(".maint");

        if let Ok(maint_file_contents) = fs::read_to_string(maint_file_path).await {
            Ok(Self {
                deploy_dir: maint_file_contents.trim().into(),
            })
        } else {
            Ok(Self::default())
        }
    }
}
