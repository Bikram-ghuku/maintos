import { useEffect, useState } from "react";
import "./deployments_grid.scss";
import { useAuthContext } from "../../utils/auth";
import { makeRequest } from "../../utils/backend";
import { Link } from "react-router-dom";
import type { Deployment } from "../../types/deployments";

function DeploymentsGrid(props: {
	deploymentState: ReturnType<typeof useState<Deployment[] | null>>;
}) {
	const auth = useAuthContext();
	const [message, setMessage] = useState<string>("");
	const [deployments, setDeployments] = props.deploymentState;

	const fetchDeployments = async () => {
		setMessage("Fetching deployments...");
		const resp = await makeRequest("deployments", "get", null, auth.jwt);

		if (resp.status == "success") {
			setDeployments(resp.data);
			setMessage(resp.message);
		} else {
			setMessage(
				`Error fetching deployments (${resp.status_code}): ${resp.message}`,
			);
		}
	};

	useEffect(() => {
		if (auth.isAuthenticated && deployments === null) {
			fetchDeployments();
		}
	}, []);

	return (
		<div className="deployments">
			<p className="message">
				<i>{message}</i>
			</p>
			{deployments !== null && (
				<div className="deployments-grid">
					{deployments!.map((deployment) => (
						<div className="deployment-card">
							<Link to={`p/${deployment.deployment_dir}`}>
								{deployment.repo_name}
							</Link>
						</div>
					))}
				</div>
			)}
		</div>
	);
}

export default DeploymentsGrid;
