CREATE TABLE IF NOT EXISTS cursos (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS semestres (
    id SERIAL PRIMARY KEY,
    ano SMALLINT NOT NULL, 
    periodo SMALLINT NOT NULL,
    UNIQUE(ano, periodo)
);

CREATE TABLE IF NOT EXISTS tecnologias (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS projetos (
    id SERIAL PRIMARY KEY,
    titulo VARCHAR(255) NOT NULL,
    tema VARCHAR(255) NOT NULL,
    resumo TEXT NOT NULL,
    github_url VARCHAR(255) NOT NULL,
    arquivo_path VARCHAR(255),
    readme_conteudo TEXT,
    curso_id INTEGER NOT NULL REFERENCES cursos(id) ON DELETE RESTRICT,
    semestre_id INTEGER NOT NULL REFERENCES semestres(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS projeto_aluno (
    projeto_id INTEGER NOT NULL REFERENCES projetos(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (projeto_id, user_id)
);

CREATE TABLE IF NOT EXISTS projeto_tecnologia (
    projeto_id INTEGER NOT NULL REFERENCES projetos(id) ON DELETE CASCADE,
    tecnologia_id INTEGER NOT NULL REFERENCES tecnologias(id) ON DELETE CASCADE,
    PRIMARY KEY (projeto_id, tecnologia_id)
);

CREATE INDEX IF NOT EXISTS idx_projetos_curso ON projetos(curso_id);
CREATE INDEX IF NOT EXISTS idx_projetos_semestre ON projetos(semestre_id);
CREATE INDEX IF NOT EXISTS idx_projeto_aluno_user ON projeto_aluno(user_id);
