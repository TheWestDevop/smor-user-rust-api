-- Your SQL goes here

CREATE TABLE smor_chef_profiles (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR(255) NOT NULL,
  dish VARCHAR(255) NOT NULL,
  details TEXT NOT NULL,
  rating Int NOT NULL DEFAULT 1,
  icon VARCHAR(255) NOT NULL,
  experience VARCHAR(255) NOT NULL,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);