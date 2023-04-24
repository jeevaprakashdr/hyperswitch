-- Your SQL goes here
ALTER TABLE merchant_account
ADD COLUMN IF NOT EXISTS created_at TIMESTAMP NOT NULL DEFAULT now(),
ADD COLUMN IF NOT EXISTS modified_at TIMESTAMP NOT NULL DEFAULT now();

ALTER TABLE merchant_connector_account
ADD COLUMN IF NOT EXISTS created_at TIMESTAMP NOT NULL DEFAULT now(),
ADD COLUMN IF NOT EXISTS modified_at TIMESTAMP NOT NULL DEFAULT now();


ALTER TABLE customers
ADD COLUMN IF NOT EXISTS modified_at TIMESTAMP NOT NULL DEFAULT now();
