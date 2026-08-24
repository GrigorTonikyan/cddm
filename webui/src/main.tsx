import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

import { Win2xManagerProvider } from "./components/ui/win2x-manager/context/win2x-manager-context";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Win2xManagerProvider>
      <App />
    </Win2xManagerProvider>
  </React.StrictMode>,
);
