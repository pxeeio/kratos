CREATE TABLE IF NOT EXISTS exercises (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    external_id SMALLINT UNIQUE,
    ulid VARCHAR UNIQUE DEFAULT generate_ulid() NOT NULL,
    type VARCHAR NOT NULL,
    target_muscle_group_id INTEGER REFERENCES muscle_groups(id) ON DELETE CASCADE,
    name VARCHAR UNIQUE NOT NULL,
    name_alternative VARCHAR,
    description TEXT,
    equipment VARCHAR REFERENCES exercise_equipment(name) ON DELETE CASCADE,
    mechanic VARCHAR,
    force VARCHAR,
    measurement VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
);
