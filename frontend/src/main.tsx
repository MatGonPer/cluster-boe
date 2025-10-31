import React from 'react'
import ReactDOM from 'react-dom/client'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'

import Register from './pages/Register.tsx' 
import Login from './pages/Login.tsx'
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
    path: "/forgot-password",
    element: <div>Página de Recuperação de Senha (a ser feita)</div>,
  },
]);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
