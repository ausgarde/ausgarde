CREATE SCHEMA ausgarde;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE ausgarde.login_type AS ENUM ('email', 'google');
CREATE TABLE ausgarde.domain_manager (
	id UUID PRIMARY KEY,
	name TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	email_verification_code TEXT,
	email_verified BOOLEAN NOT NULL DEFAULT FALSE,
	email_verified_at TIMESTAMP
);
CREATE TABLE ausgarde.domain (
	id UUID PRIMARY KEY,
	name TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	manager_id UUID NOT NULL REFERENCES ausgarde.domain_manager(id) ON DELETE CASCADE
);
CREATE TABLE ausgarde.user (
	id UUID PRIMARY KEY,
	sid TEXT,
	name TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	email_verified BOOLEAN NOT NULL DEFAULT FALSE,
	new_email TEXT,
	email_verification_code TEXT,
	password TEXT,
	login_type ausgarde.login_type NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	domain_id UUID NOT NULL REFERENCES ausgarde.domain(id) ON DELETE CASCADE,
	CONSTRAINT unique_username_domain UNIQUE (name, domain_id),
	CONSTRAINT unique_email_domain UNIQUE (email, domain_id),
	CONSTRAINT unique_server_id_domain UNIQUE (sid, domain_id)
);
CREATE TABLE ausgarde.role (
	id TEXT PRIMARY KEY,
	name TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	-- If this is null, it's a global role.
	domain_id UUID REFERENCES ausgarde.domain(id) ON DELETE CASCADE,
	CONSTRAINT unique_role_name_domain UNIQUE (name, domain_id)
);
CREATE TABLE ausgarde.domain_credentials (
	id UUID PRIMARY KEY,
	domain_id UUID NOT NULL REFERENCES ausgarde.domain(id) ON DELETE CASCADE,
	name TEXT NOT NULL,
	secret TEXT NOT NULL,
	data JSONB,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE TABLE ausgarde.session (
	id UUID PRIMARY KEY,
	user_id UUID NOT NULL REFERENCES ausgarde.user(id) ON DELETE CASCADE,
	ip_addr INET NOT NULL,
	country TEXT NOT NULL,
	user_agent TEXT NOT NULL,
	-- If null, then it's a global session.
	domain_id UUID REFERENCES ausgarde.domain(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE TABLE ausgarde.password_reset (
	id BIGSERIAL,
	user_id UUID REFERENCES ausgarde.user(id) ON DELETE CASCADE,
	domain_id UUID REFERENCES ausgarde.domain(id) ON DELETE CASCADE,
	code TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	PRIMARY KEY (id, user_id, domain_id)
);
INSERT INTO ausgarde.role (id, name)
VALUES ('admin', 'Administrator');