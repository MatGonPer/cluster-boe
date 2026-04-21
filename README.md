# Plataforma de Submissão de Projetos Integradores (Cluster BOE)

## Sobre o Projeto

Este projeto é uma plataforma Full-Stack projetada para gerenciar a submissão, processamento e armazenamento de projetos acadêmicos.

A arquitetura foi desenhada para alta performance. O Backend suporta o recebimento de arquivos pesados de forma assíncrona, utilizando streams `multipart/form-data` para garantir baixo consumo de memória RAM, além de possuir extração de documentação em memória e compressão agressiva. O Frontend fornece uma interface reativa, tipada e otimizada para lidar com uploads de arquivos complexos e rotas autenticadas.

---

## Arquitetura e Stack Tecnológico

O ecossistema é baseado em contêineres Docker, separando claramente as responsabilidades entre a API Rest e a interface do cliente.

### Backend (Rust)

| Categoria | Tecnologia |
|---|---|
| **Framework Web** | `axum` (Roteamento HTTP) com motor assíncrono `tokio` |
| **Banco de Dados** | PostgreSQL acessado via `sqlx` (ORM/Query Builder estrito) |
| **Segurança** | `jsonwebtoken` (JWT) e `bcrypt` (Hash de senhas) |
| **Processamento de Arquivos** | `axum` multipart, `zip` (Extração em memória) e `flate2` (Compressão DEFLATE) |
| **Sistema** | `uuid` (Nomenclatura única) e `dotenvy` (Variáveis de ambiente) |

### Frontend (React & TypeScript)

| Categoria | Tecnologia |
|---|---|
| **Core** | React.js com TypeScript (Tipagem estrita para mapeamento da API do Rust) |
| **Build Tool** | Vite (Empacotador ultrarrápido para desenvolvimento e build) |
| **Client HTTP** | Axios (Interceptação de tokens JWT e gestão do header `multipart/form-data`) |
| **Roteamento** | React Router DOM (Navegação SPA) |
| **Formulários** | React Hook Form (Otimização de renderização em formulários complexos) |

---

## Instruções de Execução (Setup de Desenvolvimento)

### 1. Pré-requisitos

Certifique-se de ter as seguintes ferramentas instaladas na máquina:

- Git
- Docker e Docker Compose
- Node.js (v18+) e NPM/Yarn — necessário apenas para desenvolvedores atuando no Frontend

### 2. Clonagem do Repositório

```bash
git clone git@github.com:MatGonPer/cluster-boe.git
cd cluster-boe
```

---

### 3. Configuração e Execução: Equipe de Backend (Rust)

**Configuração do `.env`:** No diretório `/backend`, crie um arquivo `.env` contendo:

```env
DATABASE_URL=postgres://postgres:postgres@db:5432/hub_projetos
JWT_SECRET=chave_secreta_jwt_desenvolvimento_123
PORT=8000
```

**Executando (Linux):**

```bash
cd backend
sudo docker compose up -d --build backend db
sudo docker compose exec backend sqlx migrate run
```

**Executando (Windows 10/11 com WSL2):**

```powershell
cd backend
docker compose up -d --build backend db
docker compose exec backend sqlx migrate run
```

---

### 4. Configuração e Execução: Equipe de Frontend (React)

A equipe de interface precisa que a API do backend esteja rodando para testar as requisições, mas o desenvolvimento visual ocorre no ambiente do Node/Vite.

**Configuração do `.env`:** No diretório `/frontend`, crie um arquivo `.env` com o apontamento para a API em Rust:

```env
VITE_API_URL=http://localhost:8000
```

**Opção A — Via Docker (Ambiente Unificado):**

Para rodar tudo via contêineres sem instalar o Node na máquina:

```bash
# Linux
sudo docker compose up -d --build frontend

# Windows (WSL2)
docker compose up -d --build frontend
```

A aplicação estará disponível em `http://localhost:5173`.

**Opção B — Localmente (Recomendado para Dev Frontend):**

Para maior velocidade de recarregamento via Hot Module Replacement (HMR):

```bash
cd frontend
npm install
npm run dev
```

---

## Regras de Integração (Frontend -> Backend)

**1. Upload de Projetos**

A rota de submissão no backend exige envio via `FormData`. A equipe de Frontend deve garantir que o header `Content-Type` **não seja forçado manualmente** como `application/json` ao enviar arquivos `.zip` ou a capa do projeto. O Axios configurará o `boundary` do multipart automaticamente ao receber um objeto `FormData`.

**2. Autenticação**

O token recebido no login deve ser armazenado (preferencialmente em cookies `HttpOnly` ou LocalStorage seguro) e anexado no header de todas as requisições subsequentes:

```
Authorization: Bearer <token>
```

---

## Estrutura do Repositório

```
cluster-boe/
├── backend/                    # API em Rust (Axum)
│   ├── src/
│   │   ├── handlers/           # Controladores de rotas e validação
│   │   └── models/             # Structs de domínio e comunicação com o banco
│   └── migrations/             # Esquemas versionados do PostgreSQL
├── frontend/                   # Interface (React + TypeScript)
│   └── src/
│       ├── components/         # Componentes visuais reutilizáveis
│       ├── pages/              # Telas principais do sistema
│       └── services/           # Configuração do Axios e chamadas à API
└── docker-compose.yaml         # Orquestração unificada da infraestrutura
```
