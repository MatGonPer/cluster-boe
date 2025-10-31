import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";

import "./index.css";

import Register from "./pages/Register";

const router = createBrowserRouter([
  {
    path: "/",
    element: <div>Landing Page</div>,
  },
  {
    path: "/register",
    element: <Register />,
  },
]);

const rootElement = document.getElementById("root")!;
ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
);
