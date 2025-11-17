import React from 'react'
import ReactDOM from 'react-dom/client'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'

import Register from './pages/Register.tsx' 
import Login from './pages/Login.tsx'
import Forgot_Confirmation from './pages/Forgot_Confirmation.tsx'
import Forgot_New_Password from './pages/Forgot_New_Password.tsx'
import './index.css'

const router = createBrowserRouter([
  {
    path: "/",
    element: <div>Página Inicial (Dashboard)</div>,
    errorElement: <div>Erro 404 - Página Não Encontrada</div>,
  },
  {
    path: "/register",
    element: <Register />, 
  },
  {
    path: "/login",
    element: <Login />,
  },
  {
    path: "/forgot-password-confirmation",
    element: <Forgot_Confirmation />,
    path: "/forgot-password-new-password",
    element: <Forgot_New_Password />,
  },
]);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
