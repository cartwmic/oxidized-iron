CREATE SCHEMA data AUTHORIZATION postgres;

CREATE TYPE data.set_kind AS ENUM (
    'Normal',
    'Myo',
    'LengthenedPartial',
    'Drop'
);

CREATE TYPE data.note_kind AS ENUM (
    'Set',
    'Exercise',
    'Workout',
    'Routine'
);

CREATE TYPE data.implement AS ENUM (
    'Bodyweight',
    'Barbell',
    'Dumbbell',
    'MachineWeightStack',
    'Cable',
    'MachinePlateLoaded'
);

CREATE TABLE data.lifting_log_entry (
  id BIGSERIAL PRIMARY KEY,
  rep_count integer NOT NULL,
  set_kind data.set_kind NOT NULL,
  rpe integer NOT NULL,
  exercise integer NOT NULL,
  workout integer NOT NULL,
  routine integer NOT NULL,
  created_at timestamptz DEFAULT NOW() NOT NULL
);

CREATE TABLE data.Notes (
  id BIGSERIAL PRIMARY KEY,
  content varchar NOT NULL,
  note_kind data.note_kind NOT NULL,
  lifting_log_entry integer NOT NULL,
  created_at timestamptz DEFAULT NOW() NOT NULL,
  updated_at timestamptz DEFAULT NOW() NOT NULL
);

CREATE TABLE data.Exercise (
  id BIGSERIAL PRIMARY KEY,
  name varchar NOT NULL,
  implement data.implement NOT NULL,
  created_at timestamptz DEFAULT NOW() NOT NULL,
  updated_at timestamptz DEFAULT NOW() NOT NULL
);

CREATE TABLE data.Workout (
  id BIGSERIAL PRIMARY KEY,
  name varchar NOT NULL,
  description varchar,
  created_at timestamptz DEFAULT NOW() NOT NULL,
  updated_at timestamptz DEFAULT NOW() NOT NULL
);

CREATE TABLE data.Routine (
  id BIGSERIAL PRIMARY KEY,
  name varchar NOT NULL,
  description varchar,
  created_at timestamptz DEFAULT NOW() NOT NULL,
  updated_at timestamptz DEFAULT NOW() NOT NULL
);

CREATE TABLE data.workout_exercise (
  id BIGSERIAL PRIMARY KEY,
  workout integer NOT NULL,
  exercise integer NOT NULL,
  created_at timestamptz DEFAULT NOW() NOT NULL
);

CREATE TABLE data.routine_workout (
  id BIGSERIAL PRIMARY KEY,
  routine integer NOT NULL,
  workout integer NOT NULL,
  created_at timestamptz DEFAULT NOW() NOT NULL
);

ALTER TABLE data.lifting_log_entry ADD FOREIGN KEY (exercise) REFERENCES data.Exercise (id);

ALTER TABLE data.lifting_log_entry ADD FOREIGN KEY (workout) REFERENCES data.Workout (id);

ALTER TABLE data.lifting_log_entry ADD FOREIGN KEY (routine) REFERENCES data.Routine (id);

ALTER TABLE data.Notes ADD FOREIGN KEY (lifting_log_entry) REFERENCES data.lifting_log_entry (id);

ALTER TABLE data.workout_exercise ADD FOREIGN KEY (workout) REFERENCES data.Workout (id);

ALTER TABLE data.workout_exercise ADD FOREIGN KEY (exercise) REFERENCES data.Exercise (id);

ALTER TABLE data.routine_workout ADD FOREIGN KEY (routine) REFERENCES data.Routine (id);

ALTER TABLE data.routine_workout ADD FOREIGN KEY (workout) REFERENCES data.Workout (id);
