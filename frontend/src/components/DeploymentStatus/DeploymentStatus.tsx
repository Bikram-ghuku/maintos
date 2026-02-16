import { useEffect, useState } from "react";
import { useAuthContext } from "../../utils/auth";
import { makeRequest } from "../../utils/backend";
import "./deployment_status.scss";
import type { IEndpointTypes } from "../../types/backend";
import { capitalize } from "../../utils/utils";

function DeploymentStatus({ projectName }: { projectName?: string }) {
  const auth = useAuthContext();

  const [deploymentStatus, setDeploymentStatus] =
    useState<IEndpointTypes["get_status"]["response"]>();
  const [message, setMessage] = useState<string>("");

  const fetchDeploymentStatus = async () => {
    if (!projectName) {
      setMessage("Project name not found.");
      return;
    }
    setMessage("Fetching deployment status...");
    const resp = await makeRequest(
      "get_status",
      "post",
      { project_name: projectName },
      auth.jwt,
    );

    if (resp.status == "success") {
      setDeploymentStatus(resp.data);
      setMessage("");
    } else {
      setMessage(
        `Error fetching project status (${resp.status_code}): ${resp.message}`,
      );
    }
  };

  useEffect(() => {
    if (auth.isAuthenticated) {
      fetchDeploymentStatus();
    }
  }, []);

  return (
    <div className="deployment-status-container">
      <div className="header">
        <h2>Deployment Status</h2>
        <button className="reload-button" onClick={fetchDeploymentStatus}>
          Reload Status
        </button>
      </div>

      {message && <p className="message">{message}</p>}

      <div className="container-grid">
        {deploymentStatus &&
          deploymentStatus.length > 0 &&
          deploymentStatus.map((container) => (
            <div key={container.container} className="container-info">
              <h3 className="container-name">{container.container}</h3>
              <p>Status: {container.status}</p>
              <p className={"status-indicator " + container.state}>{capitalize(container.state)}</p>
            </div>
          ))}
      </div>
    </div>
  );
}

export default DeploymentStatus;
