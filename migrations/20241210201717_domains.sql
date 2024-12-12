CREATE TABLE public.domain (
	id UUID PRIMARY KEY,
	name TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
	owner_id UUID NOT NULL REFERENCES ausgarde.users (id) ON DELETE CASCADE
);