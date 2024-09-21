CREATE TABLE
    client (
        client_id SERIAL PRIMARY KEY,
        name VARCHAR(20) NOT NULL
    );

CREATE TYPE STATUS AS ENUM ('timeout', 'success');

CREATE TABLE
    transaction (
        transaction_id SERIAL PRIMARY KEY,
        client_id SERIAL NOT NULL,
        value REAL NOT NULL,
        description VARCHAR(20) NOT NULL,
        when_started TIMESTAMPTZ NOT NULL,
        when_finished TIMESTAMPTZ,
        status STATUS,
        CONSTRAINT fk_transaction_client FOREIGN KEY (client_id) REFERENCES client (client_id)
    );