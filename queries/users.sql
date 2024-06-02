--! find_by_username
SELECT
    id,
    email,
    username,
    password_hash
FROM users
WHERE username = :username;

--! find_by_id
SELECT
    id,
    email,
    username,
    password_hash
FROM users
WHERE id = :id;

--! insert
INSERT INTO users(email, username, password_hash)
VALUES (:email, :username, :password_hash);
