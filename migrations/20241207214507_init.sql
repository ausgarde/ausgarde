CREATE SCHEMA ausgarde;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE public.request_type AS ENUM ('email_verification', 'password_reset');
CREATE TABLE ausgarde.users (
	id UUID PRIMARY KEY,
	email TEXT NOT NULL UNIQUE,
	email_verified BOOLEAN DEFAULT FALSE,
	name TEXT NOT NULL,
	new_email TEXT,
	password TEXT NOT NULL,
	created_at TIMESTAMP DEFAULT NOW(),
	updated_at TIMESTAMP DEFAULT NOW()
);
CREATE TABLE ausgarde.email_requests (
	-- This is the same as the user id
	-- This ensures that the user only has 1 request at a time
	id BIGSERIAL PRIMARY KEY,
	user_id UUID NOT NULL,
	type public.request_type NOT NULL,
	code TEXT NOT NULL,
	created_at TIMESTAMP DEFAULT NOW(),
	expires_at TIMESTAMP DEFAULT NOW(),
	FOREIGN KEY (user_id) REFERENCES ausgarde.users(id)
);
CREATE TABLE ausgarde.sessions (
	id TEXT,
	user_id UUID REFERENCES ausgarde.users(id),
	issued_at TIMESTAMP DEFAULT NOW(),
	last_used TIMESTAMP DEFAULT NOW(),
	expires_at TIMESTAMP,
	ip_addr INET,
	user_agent TEXT,
	country TEXT,
	PRIMARY KEY (id, user_id)
);