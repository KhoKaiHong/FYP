-- Insert data into users
INSERT INTO users (ic_number, password, name, email, phone_number, blood_type, eligibility, state_id, district_id) VALUES
('900101-01-1234', 'password123', 'John Doe', 'john.doe@example.com', '60123456789', 'A+', 'Eligible', 1, 1),
('900102-02-2345', 'password123', 'Jane Smith', 'jane.smith@example.com', '60123456790', 'B+', 'Ineligible', 1, 1),
('900103-03-3456', 'password123', 'Ali Bin Ahmad', 'ali.ahmad@example.com', '60123456791', 'O-', 'Eligible', 1, 1);

-- Insert data into event_organisers
INSERT INTO event_organisers (email, password, name, phone_number) VALUES
('organiser1@example.com', 'password123', 'Blood Organiser 1', '60123456795'),
('organiser2@example.com', 'password123', 'Blood Organiser 2', '60123456796'),
('organiser3@example.com', 'password123', 'Blood Organiser 3', '60123456797');

-- Insert data into blood_donation_events
INSERT INTO blood_donation_events (address, start_time, end_time, max_attendees, facility_id, state_id, district_id, organiser_id) VALUES
('Jalan Hospital, Johor Bahru', '2024-10-15 08:00:00', '2024-10-15 16:00:00', 100, 1, 1, 1, 1),
('Jalan Langgar, Alor Setar', '2024-10-18 09:00:00', '2024-10-18 17:00:00', 150, 2, 1, 1, 2),
('Jalan Tun Razak, Kuala Lumpur', '2024-10-20 10:00:00', '2024-10-20 18:00:00', 200, 3, 1, 1, 3);

-- Insert data into registrations
INSERT INTO registrations (status, event_id, user_id) VALUES
('Registered', 1, 1000),
('Registered', 2, 1001),
('Registered', 3, 1002);

-- Insert data into donation_history
INSERT INTO donation_history (user_id, event_id) VALUES
(1000, 1),
(1001, 2),
(1002, 3);
