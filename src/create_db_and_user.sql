-- Создание базы
DO $$
BEGIN
   IF NOT EXISTS (
       SELECT FROM pg_database WHERE datname = 'rust'
   ) THEN
       CREATE DATABASE rust;
   END IF;
END
$$;

-- Создание пользователя
DO $$
BEGIN
   IF NOT EXISTS (
       SELECT FROM pg_roles WHERE rolname = 'admin'
   ) THEN
       CREATE ROLE admin WITH LOGIN PASSWORD '2004';
   END IF;
END
$$;

-- Назначим права на базу
GRANT ALL PRIVILEGES ON DATABASE rust TO admin;

-- Подключаемся к базе
\connect rust

-- Создание таблиц
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100),
    email TEXT UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    title TEXT,
    body TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Назначим владельца таблиц (опционально)
ALTER TABLE users OWNER TO admin;
ALTER TABLE posts OWNER TO admin;
