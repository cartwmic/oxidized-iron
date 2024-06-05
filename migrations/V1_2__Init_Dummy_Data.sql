INSERT INTO data.Routine (name, description) VALUES 
  ('PPL', 'My Push Pull Legs routine'),
  ('Upper Lower', 'My Upper Lower routine');

INSERT INTO data.Workout (name, description) VALUES 
  ('Push', 'Push workout'),
  ('Pull', 'Pull workout'),
  ('Legs', 'Legs workout'),
  ('Upper', 'Upper workout'),
  ('Lower', 'Lower workout');
  
INSERT INTO data.Exercise (name, implement) VALUES 
  ('Bench Press', 'Barbell'),
  ('Pull Ups', 'Bodyweight'),
  ('Hack Squat', 'MachinePlateLoaded'),
  ('Machine Row', 'MachineWeightStack'),
  ('Romanian Deadlifts', 'Dumbbell');

INSERT INTO data.workout_exercise (workout, exercise) VALUES 
  (1, 1),
  (2, 2),
  (3, 3),
  (4, 4),
  (5, 5);


INSERT INTO data.routine_workout (routine, workout) VALUES 
  (1, 1),
  (1, 2),
  (1, 3),
  (2, 4),
  (2, 5);

INSERT INTO data.lifting_log_entry (rep_count, set_kind, rpe, exercise, workout, routine) VALUES
  (8, 'Normal', 9, 1, 1, 1),
  (10, 'Normal', 8, 4, 4, 2);
