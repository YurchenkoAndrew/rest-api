CREATE TABLE posts
(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    created_at VARCHAR(250),
    updated_at VARCHAR(250),
    user_id INT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);