CREATE SCHEMA Steam;

CREATE TABLE Steam.Accounts (
    id SERIAL PRIMARY KEY,
    username VARCHAR(45) UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE Steam.Wallets (
    id SERIAL PRIMARY KEY,
    account_id INTEGER UNIQUE NOT NULL,
    balance NUMERIC(10,2) NOT NULL DEFAULT 0,
    FOREIGN KEY (account_id) REFERENCES Steam.Accounts(id) ON DELETE CASCADE
);

CREATE TABLE Steam.Products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(45) NOT NULL,
    creator INTEGER NOT NULL,
    price NUMERIC(10,2) NOT NULL,
    FOREIGN KEY (creator) REFERENCES Steam.Accounts(id) ON DELETE CASCADE
);

CREATE TABLE Steam.Transactions (
    id SERIAL PRIMARY KEY,
    wallet_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    total_cost NUMERIC(10,2) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (wallet_id) REFERENCES Steam.Wallets(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES Steam.Products(id) ON DELETE CASCADE
);

CREATE TABLE Steam.Copies (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    owner_id INTEGER NOT NULL,
    transaction_id INTEGER NOT NULL,
    FOREIGN KEY (product_id) REFERENCES Steam.Products(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES Steam.Accounts(id) ON DELETE CASCADE,
    FOREIGN KEY (transaction_id) REFERENCES Steam.Transactions(id),
    UNIQUE (product_id, owner_id)
);
