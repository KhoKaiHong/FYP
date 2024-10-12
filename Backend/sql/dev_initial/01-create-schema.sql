---- Base app schema

-- Users
CREATE TABLE "user" (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  username varchar(128) NOT NULL UNIQUE,

  password varchar(256) NOT NULL
);

-- Task
CREATE TABLE task (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  title varchar(256) NOT NULL
);

-- App schema
-- INTEGER is i32, BIGINT is i64

-- States
CREATE TABLE "states" (
    id INTEGER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL
);

-- Districts
CREATE TABLE "districts" (
    id INTEGER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    state_id INTEGER REFERENCES states(id)
);

CREATE TYPE eligibility_status AS ENUM ('Eligible', 'Ineligible', 'Ineligible - Condition');

-- Users
CREATE TABLE "users" (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  ic_number TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  phone_number TEXT NOT NULL UNIQUE,
  blood_type TEXT,
  eligibility eligibility_status NOT NULL DEFAULT 'Eligible', -- Enum type
  state_id INTEGER REFERENCES states(id) NOT NULL,
  district_id INTEGER REFERENCES districts(id) NOT NULL
);

-- Blood Collection Facilities
CREATE TABLE "blood_collection_facilities" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    phone_number TEXT NOT NULL UNIQUE,
    state_id INTEGER REFERENCES states(id) NOT NULL
);

-- Event Organisers
CREATE TABLE "event_organisers" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL UNIQUE
);

-- Donation Events
CREATE TABLE "blood_donation_events" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    address TEXT NOT NULL,
    start timestamp NOT NULL,
    end timestamp NOT NULL,
    max_attendees INTEGER NOT NULL,
    facility_id BIGINT REFERENCES blood_collection_facilities(id) NOT NULL,
    state_id INTEGER REFERENCES states(id) NOT NULL,
    district_id INTEGER REFERENCES districts(id) NOT NULL
);

-- Events to organisers
CREATE TABLE "event_organisers_events" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    event_id BIGINT REFERENCES blood_donation_events(id) NOT NULL,
    organiser_id BIGINT REFERENCES event_organisers(id) NOT NULL
);

CREATE TYPE registration_status AS ENUM ('Registered', 'Absent', 'Attended');

-- Event Registration
CREATE TABLE "registrations" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    status registration_status NOT NULL DEFAULT 'Registered', 
    event_id INTEGER REFERENCES blood_donation_events(id) NOT NULL,
    user_id BIGINT REFERENCES users(id) NOT NULL,
    registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Donation History
CREATE TABLE "donation_history" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) NOT NULL,
    -- Can be null as donations can be done at non-events
    event_id INTEGER REFERENCES blood_donation_events(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- User Login Session
CREATE TABLE "user_sessions" (
    id UUID PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) NOT NULL,
);

-- Blood Collection Facilities Login Session
CREATE TABLE "facility_sessions" (
    id UUID PRIMARY KEY,
    facility_id BIGINT REFERENCES blood_collection_facilities(id) NOT NULL,
);

-- Organiser Login Session
CREATE TABLE "organiser_sessions" (
    id UUID PRIMARY KEY,
    organiser_id BIGINT REFERENCES event_organisers(id) NOT NULL,
);