-- Your SQL goes here

CREATE TABLE smor_chef_profiles (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR(255) NOT NULL,
  nickname VARCHAR(255) NOT NULL,
  dish VARCHAR(255) NOT NULL,
  dish_cost VARCHAR(255) NOT NULL,
  details TEXT NOT NULL,
  rating Int NOT NULL DEFAULT 1,
  icon VARCHAR(255) NOT NULL,
  experience VARCHAR(255) NOT NULL,
  state VARCHAR(255) NOT NULL,
  lga VARCHAR(255) NOT NULL,
  verification_status boolean NOT NULL DEFAULT false,
  next_of_kin_full_name VARCHAR(255) NOT NULL,
  next_of_kin_address VARCHAR(255) NOT NULL,
  next_of_kin_phone VARCHAR(255) NOT NULL,
  next_of_kin_relationship VARCHAR(255) NOT NULL,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);