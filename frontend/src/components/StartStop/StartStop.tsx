import { useState } from "react";
import { useAuthContext } from "../../utils/auth";
import { makeRequest } from "../../utils/backend";
import "./start_stop.scss";

function StartStopDeployment({ projectName }: { projectName?: string }) {
    const auth = useAuthContext();
    const [message, setMessage] = useState<string>("");
    const [disabled, setDisabled] = useState<boolean>(false);

    const start = async () => {
        if (!projectName) {
            setMessage("Project name not found.");
            return;
        }
        setMessage("Starting deployment...");
        setDisabled(true);
        const resp = await makeRequest(
            `${projectName}/start`,
            "post",
            null,
            auth.jwt,
        );

        if (resp.status == "success") {
            setMessage("");
            setDisabled(false);
        }
        else {
            setMessage(
                `Error starting deployment (${resp.status_code}): ${resp.message}`,
            );
            setDisabled(false);
        }
    };

    const stop = async () => {
        if (!projectName) {
            setMessage("Project name not found.");
            return;
        }
        setMessage("Stopping deployment...");
        setDisabled(true);
        const resp = await makeRequest(
            `${projectName}/stop`,
            "post",
            null,
            auth.jwt,
        );

        if (resp.status == "success") {
            setMessage("");
            setDisabled(false);
        }
        else {
            setMessage(
                `Error stopping deployment (${resp.status_code}): ${resp.message}`,
            );
            setDisabled(false);
        }
    };

    return (
        <div className="start-stop-container">
            {message && <p className="message">{message}</p>}

            <div className="buttons">
                <button className="start-button" onClick={start} disabled={disabled}>
                    Start
                </button>
                <button className="stop-button" onClick={stop} disabled={disabled}>
                    Stop
                </button>
            </div>
        </div>
    );
}

export default StartStopDeployment;
