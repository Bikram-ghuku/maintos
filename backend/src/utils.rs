use std::collections::HashMap;

use anyhow::anyhow;
use bollard::{Docker, query_parameters::ListContainersOptionsBuilder, models::ContainerSummary};
use tokio::fs;

use crate::{deployments::Deployment, env::EnvVars};

pub(crate) type Res<T> = Result<T, anyhow::Error>;

/// Get a list of deployments
pub async fn get_deployments(env_vars: &EnvVars) -> Res<Vec<Deployment>> {
    let deployments_dir = &env_vars.deployments_dir;

    let mut deployments = Vec::new();

    let mut dir_iter = fs::read_dir(deployments_dir).await?;
    while let Some(path) = dir_iter.next_entry().await? {
        if path.file_type().await?.is_dir() {
            let deployment_dir = path
                .file_name()
                .into_string()
                .map_err(|_| anyhow!("Invalid project name"))?;
            
            let git_path = deployments_dir.join(&deployment_dir).join(".git");
            if git_path.exists() {
                let deployment = Deployment::from_deployment_dir(env_vars, &deployment_dir).await?;

                if deployment.repo_owner == env_vars.gh_org_name {
                    deployments.push(deployment);
                }
            }
        }
    }

    Ok(deployments)
}

/// Get a list of containers under a deployment
pub async fn get_containers(docker: &Docker, deployment_dir: &str) -> Res<Vec<ContainerSummary>> {
    let mut filter = HashMap::new();
    filter.insert("label".to_string(), vec![format!("com.docker.compose.project.working_dir={}", deployment_dir)]);

    let containers = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::default().all(true)
            .filters(&filter)
            .build(),
        ))
        .await?;

    Ok(containers)
}
