import type { Deployment } from "./deployments";

export type AllowedBackendMethods = "get" | "post";

export interface IOkResponse<T> {
	status: "success";
	message: string;
	status_code: 200;
	data: T;
}

export interface IErrorResponse {
	status: "error";
	message: string;
	status_code: number | string;
}

export type BackendResponse<T> = IOkResponse<T> | IErrorResponse;

export interface IEndpointTypes {
	oauth: {
		request: {
			code: string;
		};
		response: {
			token: string;
		};
	};
	profile: {
		request: null;
		response: {
			username: string;
			token: string;
		};
	};
	deployments: {
		request: null;
		response: Deployment[];
	};
	get_env: {
		request: {
			project_name: string;
		}
		response: Record<string, string>
	};
	get_status: {
		request: {
			project_name: string;
		}
		response: {
			container: string;
			state: "" | "unknown" | "created" | "restarting" | "running" | "removing" | "paused" | "exited" | "dead";
			status: string;
		}[]
	};
}
