import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';

export default function Login() {
    
    const [email, setEmail] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setError(null);

        try {
            const response = await fetch('http://localhost:3000/api/auth/login', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ 
                    email: email.trim(), 
                    password: password.trim() 
                }),
                credentials: 'include',
            });

            if (response.ok) {
                console.log('Login bem-sucedido, cookie HttpOnly foi definido pelo servidor!');
                navigate('/');
            } else {
                const errorData = await response.json();
                setError(errorData.error || 'Falha no login.');
            }
        } catch (err) {
            console.error('Erro de rede:', err);
            setError('Não foi possível conectar ao servidor.');
        }
    };

    return (
        <div className='text-end'>
            <h2 className='font-bold'>Login</h2>
            
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
                    />
                </div>
                
                {error && <p style={{ color: 'red' }}>{error}</p>}
                
                <button type="submit">Entrar</button>
            </form>

            <hr />

            <p className=''>Não tem uma conta? <Link to="/register">Registe-se agora!</Link></p>
            <p className='underline'><Link to="/forgot-password">Esqueceu sua senha?</Link></p>
        </div>
    );
}
