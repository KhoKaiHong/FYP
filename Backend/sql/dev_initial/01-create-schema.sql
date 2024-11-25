-- App schema
-- INTEGER is i32, BIGINT is i64
-- REAL is f32, DOUBLE PRECISION is f64

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
CREATE TYPE blood_type_enum AS ENUM ('A+', 'A-', 'B+', 'B-', 'O+', 'O-', 'AB+', 'AB-');

-- Users
CREATE TABLE "users" (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  ic_number TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  phone_number TEXT NOT NULL UNIQUE,
  blood_type blood_type_enum NOT NULL,
  eligibility eligibility_status NOT NULL DEFAULT 'Eligible',
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

-- Admins
CREATE TABLE "admins" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    name TEXT NOT NULL
);

-- Donation Events
CREATE TABLE "blood_donation_events" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    address TEXT NOT NULL,
    start_time timestamp NOT NULL,
    end_time timestamp NOT NULL,
    max_attendees INTEGER NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    facility_id BIGINT REFERENCES blood_collection_facilities(id) NOT NULL,
    organiser_id BIGINT REFERENCES event_organisers(id) NOT NULL,
    state_id INTEGER REFERENCES states(id) NOT NULL,
    district_id INTEGER REFERENCES districts(id) NOT NULL
);

CREATE TYPE event_request_status AS ENUM ('Pending', 'Approved', 'Rejected');

CREATE TABLE "new_blood_donation_events_requests" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    address TEXT NOT NULL,
    start_time timestamp NOT NULL,
    end_time timestamp NOT NULL,
    max_attendees INTEGER NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    status event_request_status NOT NULL DEFAULT 'Pending',
    rejection_reason TEXT,
    facility_id BIGINT REFERENCES blood_collection_facilities(id) NOT NULL,
    organiser_id BIGINT REFERENCES event_organisers(id) NOT NULL,
    state_id INTEGER REFERENCES states(id) NOT NULL,
    district_id INTEGER REFERENCES districts(id) NOT NULL
);

CREATE TABLE "change_blood_donation_events_requests" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    address TEXT NOT NULL,
    start_time timestamp NOT NULL,
    end_time timestamp NOT NULL,
    max_attendees INTEGER NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    status event_request_status NOT NULL DEFAULT 'Pending',
    change_reason TEXT NOT NULL,
    rejection_reason TEXT,
    event_id BIGINT REFERENCES blood_donation_events(id) NOT NULL,
    facility_id BIGINT REFERENCES blood_collection_facilities(id) NOT NULL,
    organiser_id BIGINT REFERENCES event_organisers(id) NOT NULL,
    state_id INTEGER REFERENCES states(id) NOT NULL,
    district_id INTEGER REFERENCES districts(id) NOT NULL
);

-- Event Registration

CREATE TYPE registration_status AS ENUM ('Registered', 'Absent', 'Attended');

CREATE TABLE "registrations" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    status registration_status NOT NULL DEFAULT 'Registered', 
    event_id BIGINT REFERENCES blood_donation_events(id) NOT NULL,
    user_id BIGINT REFERENCES users(id) NOT NULL,
    registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Donation History
CREATE TABLE "donation_history" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) NOT NULL,
    -- Can be null as donations can be done at non-events
    event_id BIGINT REFERENCES blood_donation_events(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Notifications

CREATE TABLE "user_notifications" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    description TEXT NOT NULL,
    redirect TEXT,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id BIGINT REFERENCES users(id) NOT NULL
);

CREATE TABLE "facility_notifications" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    description TEXT NOT NULL,
    redirect TEXT,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    facility_id BIGINT REFERENCES blood_collection_facilities(id) NOT NULL
);

CREATE TABLE "organiser_notifications" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    description TEXT NOT NULL,
    redirect TEXT,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    organiser_id BIGINT REFERENCES event_organisers(id) NOT NULL
);

CREATE TABLE "admin_notifications" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    description TEXT NOT NULL,
    redirect TEXT,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    admin_id BIGINT REFERENCES admins(id) NOT NULL
);

-- User Login Session
CREATE TABLE "user_sessions" (
    refresh_token_id UUID PRIMARY KEY,
    access_token_id UUID NOT NULL UNIQUE,
    user_id BIGINT REFERENCES users(id) NOT NULL
);

-- Blood Collection Facilities Login Session
CREATE TABLE "facility_sessions" (
    refresh_token_id UUID PRIMARY KEY,
    access_token_id UUID NOT NULL UNIQUE,
    facility_id BIGINT REFERENCES blood_collection_facilities(id) NOT NULL
);

-- Organiser Login Session
CREATE TABLE "organiser_sessions" (
    refresh_token_id UUID PRIMARY KEY,
    access_token_id UUID NOT NULL UNIQUE,
    organiser_id BIGINT REFERENCES event_organisers(id) NOT NULL
);

-- Admin Login Session
CREATE TABLE "admin_sessions" (
    refresh_token_id UUID PRIMARY KEY,
    access_token_id UUID NOT NULL UNIQUE,
    admin_id BIGINT REFERENCES admins(id) ON DELETE CASCADE NOT NULL
);