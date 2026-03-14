-- USERS
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- NOTEBOOKS
CREATE TABLE notebooks (
    id SERIAL PRIMARY KEY,
    owner_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL ,
    created_at TIMESTAMP DEFAULT NOW()
);

-- NOTES
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    notebook_id INT NOT NULL REFERENCES notebooks(id) ON DELETE CASCADE,
    owner_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    visibility TEXT DEFAULT 'private',
    last_editor_id INT REFERENCES users(id),
    updated_at TIMESTAMP DEFAULT NOW()
);