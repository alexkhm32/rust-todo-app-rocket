CREATE OR REPLACE FUNCTION set_timestamp()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
    SECURITY DEFINER
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END
$$;

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    login VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE todo_items (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL REFERENCES accounts(id),
    title VARCHAR(100) NOT NULL,
    status VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON todo_items FOR EACH ROW
    EXECUTE PROCEDURE set_timestamp();
