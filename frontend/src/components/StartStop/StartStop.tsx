import { useState } from "react";
import { useAuthContext } from "../../utils/auth";
import { makeRequest } from "../../utils/backend";
import "./start_stop.scss";

function StartStopDeployment({ projectName }: { projectName?: string }) {
    const auth = useAuthContext();
    const [message, setMessage] = useState<string>("");

    const start = async () => {
        if (!projectName) {
            setMessage("Project name not found.");
            return;
        }
        setMessage("Starting deployment...");
        const resp = await makeRequest(
            `${projectName}/start`,
            "post",
            null,
            auth.jwt,
        );

        if (resp.status == "success") {
            setMessage("");
        }
        else {
            setMessage(
                `Error starting deployment (${resp.status_code}): ${resp.message}`,
            );
        }
    };

    const stop = async () => {
        if (!projectName) {
            setMessage("Project name not found.");
            return;
        }
        setMessage("Stopping deployment...");
        const resp = await makeRequest(
            `${projectName}/stop`,
            "post",
            null,
            auth.jwt,
        );

        if (resp.status == "success") {
            setMessage("");
        }
        else {
            setMessage(
                `Error stopping deployment (${resp.status_code}): ${resp.message}`,
            );
        }
    };

    return (
        <div className="start-stop-container">
            {message && <p className="message">{message}</p>}

            <div className="buttons">
                <button className="start-button" onClick={start}>
                    Start
                </button>
                <button className="stop-button" onClick={stop}>
                    Stop
                </button>
            </div>
        </div>
    );
}

export default StartStopDeployment;
