import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';

export default function Register() {
    
    const [email, setEmail] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [confirmPassword, setConfirmPassword] = useState<string>('');
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setError(null);

        if (password !== confirmPassword) {
            setError('As senhas não coincidem.');
            return;
        }

        try {
            const response = await fetch('http://localhost:3000/api/auth/register', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ 
                    email: email.trim(), 
                    password: password.trim(),
                    confirm_password: confirmPassword.trim()
                }),
                credentials: 'include',
            });

            if (response.ok) {
                console.log('Registo bem-sucedido!');
                navigate('/login');
            } else {
                const errorData = await response.json();
                setError(errorData.error || 'Falha no registo.');
            }
        } catch (err) {
            console.error('Erro de rede:', err);
            setError('Não foi possível conectar ao servidor.');
        }
    };

    return (
        <div>
            <h1>Página de Registo</h1>
            
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
                        required
                        minLength={12}
                    />
                </div>
                <div>
                    <label htmlFor="confirmPassword">Confirme sua Senha:</label>
                    <input
                        id="confirmPassword"
                        type="password"
                        value={confirmPassword}
                        onChange={(e) => setConfirmPassword(e.target.value)}
                        required
                        minLength={12}
                    />
                </div>
                
                {error && <p style={{ color: 'red' }}>{error}</p>}
                
                <button type="submit">Registar</button>
            </form>

            <hr /> 
            
            <p>Já tem uma conta? <Link to="/login">Entre agora!</Link></p>
            <p><Link to="/forgot-password">Esqueceu sua senha?</Link></p>
        </div>
    );
}
