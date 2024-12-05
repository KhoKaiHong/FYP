-- Insert data into users
INSERT INTO users (ic_number, password, name, email, phone_number, blood_type, eligibility, state_id, district_id) VALUES
('900101-01-1234', '$argon2id$v=19$m=16,t=2,p=1$aGVsbG80dDM$wDfiMZQUyIUHkEd8m/j9Cw', 'John Doe', 'john.doe@example.com', '+6012-3456789', 'A+', 'Ineligible - Condition', 1, 1),
('900102-02-2345', '$argon2id$v=19$m=16,t=2,p=1$aGVsbG80dDM$wDfiMZQUyIUHkEd8m/j9Cw', 'Jane Smith', 'jane.smith@example.com', '+6012-3456790', 'B+', 'Eligible', 1, 1),
('900103-03-3456', '$argon2id$v=19$m=16,t=2,p=1$aGVsbG80dDM$wDfiMZQUyIUHkEd8m/j9Cw', 'Ali Bin Ahmad', 'ali.ahmad@example.com', '+6012-3456791', 'O-', 'Eligible', 1, 1);

-- Insert data into event_organisers
INSERT INTO event_organisers (email, password, name, phone_number) VALUES
('organiser1@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Blood Organiser 1', '60123456795'),
('organiser2@example.com', 'password123', 'Blood Organiser 2', '60123456796'),
('organiser3@example.com', 'password123', 'Blood Organiser 3', '60123456797');

-- Insert data into admins
INSERT INTO admins (email, password, name) VALUES
('admin1@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Admin 1'),
('admin2@example.com', 'password123', 'Admin 2'),
('admin3@example.com', 'password123', 'Admin 3');

-- Insert data into blood_donation_events
INSERT INTO blood_donation_events (location, address, start_time, end_time, max_attendees, latitude, longitude, facility_id, state_id, district_id, organiser_id) VALUES
('Mid Valley Megamall', 'Jalan Hospital, Johor Bahru', '2024-12-30 3:00:00', '2024-12-30 10:00:00', 50, 3.1732962387784367, 101.70668106095312, 1, 1, 1, 1),
('Pavilion Bukit Jalil', 'Jalan Langgar, Alor Setar', '2024-12-31 4:00:00', '2024-12-31 11:00:00', 100, 3.1742962387784367, 101.70768106095312, 2, 1, 1, 2),
('Pavilion KL', 'Jalan Tun Razak, Kuala Lumpur', '2024-12-31 5:00:00', '2024-12-31 12:00:00', 70, 3.1752962387784367, 101.70868106095312, 3, 1, 1, 3);

-- Insert data into registrations
INSERT INTO registrations (status, event_id, user_id) VALUES
('Registered', 1, 1002),
('Registered', 2, 1002),
('Registered', 3, 1002);

-- Insert data into donation_history
INSERT INTO donation_history (user_id, event_id) VALUES
(1000, 1),
(1001, 2),
(1002, 3);
