CREATE SCHEMA data AUTHORIZATION postgres;

-- CREATE ROUTINE TABLE
CREATE TABLE data.routine
(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(250) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- CREATE WORKOUT TABLE
CREATE TABLE data.workout
(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(250) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- CREATE ROUTINE WORKOUT TABLE
CREATE TABLE data.routine_workout
(
    routine_id BIGSERIAL NOT NULL,
    workout_id BIGSERIAL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);
ALTER TABLE data.routine_workout
  ADD CONSTRAINT pk_routine_workout_routine_id_workout_id PRIMARY KEY (routine_id, workout_id),
  ADD CONSTRAINT fk_routine_workout_routine_id FOREIGN KEY(routine_id) REFERENCES data.routine(id) ON DELETE CASCADE,
  ADD CONSTRAINT fk_routine_workout_workout_id FOREIGN KEY(workout_id) REFERENCES data.workout(id) ON DELETE CASCADE;
