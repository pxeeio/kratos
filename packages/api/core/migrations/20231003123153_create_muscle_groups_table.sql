CREATE TABLE IF NOT EXISTS muscle_groups (
    id SMALLSERIAL PRIMARY KEY NOT NULL,
    name VARCHAR UNIQUE NOT NULL,
    image_source VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
)
