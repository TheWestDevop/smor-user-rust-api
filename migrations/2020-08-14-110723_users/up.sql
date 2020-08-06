-- Your SQL goes here-- 
CREATE TABLE smor_users (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  phone VARCHAR(255) NOT Null,
  avatar VARCHAR(500) NOT NULL DEFAULT 'https://api.adorable.io/avatars/285/smorfarm@adorable.io.png',
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  role Int NOT Null DEFAULT 1,
  status BOOLEAN NOT NULL DEFAULT TRUE,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);