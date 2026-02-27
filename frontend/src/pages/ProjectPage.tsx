import { useEffect } from "react";
import { Header } from "../components/Common/Common";
import { useAuthContext } from "../utils/auth";
import { useParams } from "react-router-dom";
import EnvVars from "../components/EnvVars/EnvVars";
import DeploymentStatus from "../components/DeploymentStatus/DeploymentStatus";
import StartStopDeployment from "../components/StartStop/StartStop";

function ProjectPage() {
	const auth = useAuthContext();

    useEffect(() => {
        if (!auth.isAuthenticated) {
            window.location.assign("/");
        }
    }, []);

    const { projectName } = useParams();

    return (
        <>
            <Header
                title={projectName?.toUpperCase() || "Project"}
                subtitle={
                    auth.isAuthenticated
                        ? `Welcome ${auth.username}!`
                        : `Not authenticated.`
                }
            />
            {auth.isAuthenticated && <>
                <DeploymentStatus projectName={projectName} />
                <StartStopDeployment projectName={projectName} /> 
                <EnvVars projectName={projectName} />
            </>}
        </>
    );
}

export default ProjectPage;
