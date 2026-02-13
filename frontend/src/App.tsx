import { BrowserRouter, Route, Routes } from "react-router-dom";
import { AuthProvider } from "./utils/auth";
import OAuthPage from "./pages/OAuthPage";
import MainPage from "./pages/MainPage";
import ProjectPage from "./pages/ProjectPage";

function App() {
	return (
		<>
			<BrowserRouter>
				<AuthProvider>
					<Routes>
						<Route path="/" element={<MainPage />} />
						<Route path="/oauth" element={<OAuthPage />} />
						<Route
							path="/p/:projectName"
							element={<ProjectPage />}
						/>
					</Routes>
				</AuthProvider>
			</BrowserRouter>
		</>
	);
}

export default App;
