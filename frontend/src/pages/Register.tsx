import React, { useState } from "react";
import { useNavigate, Link } from "react-router-dom";

export default function Register() {
  const [email, setEmail] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [confirmPassword, setConfirmPassword] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(null);

    if (password !== confirmPassword) {
      setError("As senhas não coincidem.");
      return;
    }
    
    try {
      const response = await fetch("http://localhost:3000/api/auth/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email: email.trim(),
          password: password.trim(),
        }),
      });

      if (response.ok) {
        console.log("Registro bem-sucedido.");
        navigate("/login");
      } else {
        const errorData = await response.json();
        setError(errorData.error || "Falha no registro.");
      }
    } catch (err) {
      console.error("Erro de rede:", err);
      setError("Não foi possível conectar-se ao servidor.");
    }
  };

  return (
    <div>

      <div>

        <h2>Criar Conta</h2>
        <form onSubmit={handleSubmit}>

          <div>
            <label htmlFor="email">Email:</label>
            <input
              id="email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>

          <div>
            <label htmlFor="password">Senha:</label>
            <input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              minLength={12}
              required
            />
          </div>

          <div>
            <label htmlFor="confirmPassword">Confirmar sua senha:</label>
            <input
              id="confirmPassword"
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              minLength={12}
              required
            />
          </div>
          
          {error && <p style={{ color: "red" }}>{error}</p>}
          
          <button type="submit">Registrar</button>

        </form>

        <p>Já tem uma conta? <Link to="/login">Faça seu login aqui</Link>.</p>
        <p><Link to="/forgot-password">Esqueceu a sua senha?</Link></p>

      </div>

    </div>
  );
}
