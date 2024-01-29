-- Insert example routine
INSERT INTO data.routine (name, description) 
  VALUES ('RP - Push/Pull/Legs', 'Renaissance Periodization focused push/pull/legs routine');

-- Insert example workouts
INSERT INTO data.workout (name, description) VALUES 
  ('RP - Push', 'Renaissance Periodization focused push workout'),
  ('RP - Pull', 'Renaissance Periodization focused pull workout'),
  ('RP - Legs', 'Renaissance Periodization focused legs workout'),
  ('Generic - Legs', 'Generic legs workout');
    
-- Example attach workouts to a routine
INSERT INTO data.routine_workout (routine_id, workout_id) VALUES
  (1, 1),
  (1, 2),
  (1, 3);
