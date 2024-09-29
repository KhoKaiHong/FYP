-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'dev' OR datname = 'dev_db';
DROP DATABASE IF EXISTS dev_db;
DROP USER IF EXISTS dev;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER dev PASSWORD 'dev_only_pwd';
CREATE DATABASE dev_db owner dev ENCODING = 'UTF-8';