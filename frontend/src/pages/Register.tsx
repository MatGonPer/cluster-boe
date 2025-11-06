import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { MdEmail, MdPerson } from 'react-icons/md';
import { RiLockPasswordLine } from 'react-icons/ri';

export default function Register() {
    const [email, setEmail] = useState<string>('');
    const [username, setUsername] = useState<string>('');
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
        <div className="min-h-screen bg-white">
            <div className="min-h-screen grid grid-cols-1 md:grid-cols-2">
                {/* Left - imagem + marketing */}
                <div
                    className="hidden md:flex relative items-center h-full"
                    style={{
                        backgroundImage: "url('/src/assets/servidores.png')",
                        backgroundSize: 'cover',
                        backgroundPosition: 'center'
                    }}
                >
                    <div className="absolute inset-0 bg-gradient-to-r from-black/70 via-purple-700/50 to-black/60"></div>
                    <div className="relative z-10 px-12 lg:px-20">
                        <h1 className="text-white font-extrabold text-4xl lg:text-5xl leading-tight drop-shadow-md">
                            Faça login ou <span className="text-pink-400">Cadastre-se</span> para
                        </h1>
                        <p className="mt-6 text-white text-3xl font-bold leading-tight">ter acesso a todos os recursos!</p>
                    </div>
                </div>

                {/* Right - card */}
                <div className="flex items-center justify-center p-8 bg-gray-50">
                    <div className="w-full max-w-xl bg-white rounded-3xl shadow-2xl p-12 border border-gray-200">
                        <div className="flex flex-col items-center mb-8">
                            <img src="/src/assets/logo.png" alt="cluster boe" className="w-40 h-auto mb-6" />
                            <h2 className="text-3xl font-extrabold">Cadastre-se</h2>
                        </div>

                        <form onSubmit={handleSubmit} className="space-y-6">
                            <div>
                                <label htmlFor="username" className="sr-only">Usuário</label>
                                <div className="flex items-center bg-white border border-gray-300 rounded-full px-6 py-3 focus-within:ring-2 focus-within:ring-purple-400">
                                    <div className="mr-4"><MdPerson className="w-6 h-6 text-gray-400" /></div>
                                    <input
                                        id="username"
                                        type="text"
                                        value={username}
                                        onChange={(e) => setUsername(e.target.value)}
                                        required
                                        placeholder="Digite seu usuário"
                                        className="w-full outline-none text-base text-gray-700 placeholder-gray-400 bg-transparent"
                                    />
                                </div>
                            </div>

                            <div>
                                <label htmlFor="email" className="sr-only">Email</label>
                                <div className="flex items-center bg-white border border-gray-300 rounded-full px-6 py-3 focus-within:ring-2 focus-within:ring-purple-400">
                                    <div className="mr-4"><MdEmail className="w-6 h-6 text-gray-400" /></div>
                                    <input
                                        id="email"
                                        type="email"
                                        value={email}
                                        onChange={(e) => setEmail(e.target.value)}
                                        required
                                        placeholder="Digite seu email"
                                        className="w-full outline-none text-base text-gray-700 placeholder-gray-400 bg-transparent"
                                    />
                                </div>
                            </div>

                            <div>
                                <label htmlFor="password" className="sr-only">Senha</label>
                                <div className="flex items-center bg-white border border-gray-300 rounded-full px-6 py-3 focus-within:ring-2 focus-within:ring-purple-400">
                                    <div className="mr-4"><RiLockPasswordLine className="w-6 h-6 text-gray-400" /></div>
                                    <input
                                        id="password"
                                        type="password"
                                        value={password}
                                        onChange={(e) => setPassword(e.target.value)}
                                        required
                                        minLength={12}
                                        placeholder="Digite sua senha"
                                        className="w-full outline-none text-base text-gray-700 placeholder-gray-400 bg-transparent"
                                    />
                                </div>
                            </div>

                            <div>
                                <label htmlFor="confirmPassword" className="sr-only">Repita sua senha</label>
                                <div className="flex items-center bg-white border border-gray-300 rounded-full px-6 py-3 focus-within:ring-2 focus-within:ring-purple-400">
                                    <div className="mr-4"><RiLockPasswordLine className="w-6 h-6 text-gray-400" /></div>
                                    <input
                                        id="confirmPassword"
                                        type="password"
                                        value={confirmPassword}
                                        onChange={(e) => setConfirmPassword(e.target.value)}
                                        required
                                        minLength={12}
                                        placeholder="Repita sua senha"
                                        className="w-full outline-none text-base text-gray-700 placeholder-gray-400 bg-transparent"
                                    />
                                </div>
                            </div>

                            {error && <p className="text-base text-red-600">{error}</p>}

                            <button
                                type="submit"
                                className="w-full py-4 rounded-full text-white font-semibold text-lg bg-gradient-to-r from-purple-600 to-pink-600 shadow-md hover:opacity-95 transition"
                            >
                                Cadastrar
                            </button>
                        </form>

                        <div className="mt-8 text-center">
                            <p className="text-base text-gray-500">
                                Já tem uma conta? {' '}
                                <Link to="/login" className="font-medium text-blue-600 hover:underline">
                                    Entre agora!
                                </Link>
                            </p>

                            <Link
                                to="/login"
                                className="inline-block mt-6 px-8 py-3 rounded-full bg-teal-500 text-white font-semibold text-lg shadow-md hover:brightness-95"
                            >
                                Entrar
                            </Link>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}