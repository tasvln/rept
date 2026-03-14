-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE receipts (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    store_name TEXT,
    purchase_date DATE,
    total NUMERIC(10, 2),
    return_by DATE,
    warranty_until DATE,
    image_url TEXT,
    raw_text TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);