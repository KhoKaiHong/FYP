-- Insert data into users
INSERT INTO users (ic_number, password, name, email, phone_number, blood_type, eligibility, state_id, district_id) VALUES
('900101-01-1234', '$argon2id$v=19$m=16,t=2,p=1$aGVsbG80dDM$wDfiMZQUyIUHkEd8m/j9Cw', 'John Doe', 'john.doe@example.com', '+6012-3456789', 'A+', 'Ineligible - Condition', 1, 1),
('900102-02-2345', '$argon2id$v=19$m=16,t=2,p=1$aGVsbG80dDM$wDfiMZQUyIUHkEd8m/j9Cw', 'Jane Smith', 'jane.smith@example.com', '+6012-3456790', 'B+', 'Eligible', 1, 1),
('900103-03-3456', '$argon2id$v=19$m=16,t=2,p=1$aGVsbG80dDM$wDfiMZQUyIUHkEd8m/j9Cw', 'Ali Bin Ahmad', 'ali.ahmad@example.com', '+6012-3456791', 'O-', 'Eligible', 1, 1);

-- Insert data into event_organisers
INSERT INTO event_organisers (email, password, name, phone_number) VALUES
('organiser1@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Blood Organiser 1', '60123456795'),
('organiser2@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Blood Organiser 2', '60123456796'),
('organiser3@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Blood Organiser 3', '60123456797');

-- Insert data into admins
INSERT INTO admins (email, password, name) VALUES
('admin1@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Admin 1'),
('admin2@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Admin 2'),
('admin3@example.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$dxy5zcoYs+QaveCUO+t/8w', 'Admin 3');

-- Insert data into blood_donation_events
INSERT INTO blood_donation_events (location, address, start_time, end_time, max_attendees, latitude, longitude, facility_id, state_id, district_id, organiser_id) VALUES
('Mid Valley Megamall 1', 'Jalan Hospital, Johor Bahru', '2024-12-01 3:00:00', '2024-12-30 9:00:00', 50, 3.1712962387784367, 101.70368106095312, 1, 1, 1, 1),
('Mid Valley Megamall 2', 'Jalan Hospital, Johor Bahru', '2024-12-10 3:00:00', '2024-12-30 9:00:00', 50, 3.1722962387784367, 101.70468106095312, 1, 1, 1, 1),
('Mid Valley Megamall 3', 'Jalan Hospital, Johor Bahru', '2024-12-20 3:00:00', '2024-12-30 9:00:00', 50, 3.1732962387784367, 101.70568106095312, 1, 1, 1, 1),
('Mid Valley Megamall 4', 'Jalan Hospital, Johor Bahru', '2024-12-30 3:00:00', '2024-12-30 9:00:00', 50, 3.1742962387784367, 101.70668106095312, 1, 1, 1, 1),
('Pavilion Bukit Jalil', 'Jalan Langgar, Alor Setar', '2024-12-31 4:00:00', '2024-12-31 9:00:00', 100, 3.1742962387784367, 101.70768106095312, 2, 1, 1, 2),
('Pavilion KL', 'Jalan Tun Razak, Kuala Lumpur', '2024-12-31 5:00:00', '2024-12-31 9:00:00', 70, 3.1752962387784367, 101.70868106095312, 3, 1, 1, 3);

-- Insert data into registrations
INSERT INTO registrations (status, event_id, user_id) VALUES
('Registered', 1, 1002),
('Registered', 2, 1002),
('Registered', 3, 1002);

-- Insert data into donation_history
INSERT INTO donation_history (user_id, event_id) VALUES
(1000, 1),
(1000, 2),
(1000, 3),
(1000, 4),
(1000, 5),
(1000, 6),
(1000, NULL),
(1001, 2),
(1002, 3);

-- Insert new blood donation events requests
INSERT INTO new_blood_donation_events_requests (
    location, address, start_time, end_time, max_attendees,
    latitude, longitude, status, rejection_reason, facility_id,
    organiser_id, state_id, district_id
) VALUES
('Location 1', 'Address for Location 1, Malaysia', '2025-01-01 01:00:00', '2025-01-01 06:00:00', 50, 3.0925, 101.6495, 'Approved', NULL, 1, 1, 1, 1),
('Location 2', 'Address for Location 2, Malaysia', '2025-01-02 02:00:00', '2025-01-02 07:00:00', 100, 3.0927, 101.6496, 'Pending', NULL, 1, 2, 2, 2),
('Location 3', 'Address for Location 3, Malaysia', '2025-01-03 03:00:00', '2025-01-03 08:00:00', 150, 3.0923, 101.6497, 'Rejected', 'Insufficient resources at the facility', 1, 3, 3, 3),
('Location 4', 'Address for Location 4, Malaysia', '2025-01-04 01:00:00', '2025-01-04 06:00:00', 200, 3.0926, 101.6498, 'Approved', NULL, 1, 1, 4, 4),
('Location 5', 'Address for Location 5, Malaysia', '2025-01-05 02:00:00', '2025-01-05 07:00:00', 250, 3.0924, 101.6499, 'Pending', NULL, 1, 2, 5, 5),
('Location 6', 'Address for Location 6, Malaysia', '2025-01-06 03:00:00', '2025-01-06 08:00:00', 300, 3.0922, 101.6493, 'Rejected', 'Insufficient resources at the facility', 1, 3, 1, 6),
('Location 7', 'Address for Location 7, Malaysia', '2025-01-07 01:00:00', '2025-01-07 06:00:00', 50, 3.0928, 101.6494, 'Approved', NULL, 1, 1, 2, 7),
('Location 8', 'Address for Location 8, Malaysia', '2025-01-08 02:00:00', '2025-01-08 07:00:00', 100, 3.0921, 101.6495, 'Pending', NULL, 1, 2, 3, 8),
('Location 9', 'Address for Location 9, Malaysia', '2025-01-09 03:00:00', '2025-01-09 08:00:00', 150, 3.0929, 101.6496, 'Rejected', 'Insufficient resources at the facility', 1, 3, 4, 9),
('Location 10', 'Address for Location 10, Malaysia', '2025-01-10 01:00:00', '2025-01-10 06:00:00', 200, 3.0923, 101.6497, 'Approved', NULL, 1, 1, 5, 10),
('Location 11', 'Address for Location 11, Malaysia', '2025-01-11 02:00:00', '2025-01-11 07:00:00', 250, 3.0924, 101.6498, 'Pending', NULL, 1, 2, 1, 11),
('Location 12', 'Address for Location 12, Malaysia', '2025-01-12 03:00:00', '2025-01-12 08:00:00', 300, 3.0922, 101.6499, 'Rejected', 'Insufficient resources at the facility', 1, 3, 2, 12),
('Location 13', 'Address for Location 13, Malaysia', '2025-01-13 01:00:00', '2025-01-13 06:00:00', 50, 3.0925, 101.6493, 'Approved', NULL, 1, 1, 3, 13),
('Location 14', 'Address for Location 14, Malaysia', '2025-01-14 02:00:00', '2025-01-14 07:00:00', 100, 3.0927, 101.6494, 'Pending', NULL, 1, 2, 4, 14),
('Location 15', 'Address for Location 15, Malaysia', '2025-01-15 03:00:00', '2025-01-15 08:00:00', 150, 3.0923, 101.6495, 'Rejected', 'Insufficient resources at the facility', 1, 3, 5, 15),
('Location 16', 'Address for Location 16, Malaysia', '2025-01-16 01:00:00', '2025-01-16 06:00:00', 200, 3.0926, 101.6496, 'Approved', NULL, 1, 1, 1, 16),
('Location 17', 'Address for Location 17, Malaysia', '2025-01-17 02:00:00', '2025-01-17 07:00:00', 250, 3.0924, 101.6497, 'Pending', NULL, 1, 2, 2, 17),
('Location 18', 'Address for Location 18, Malaysia', '2025-01-18 03:00:00', '2025-01-18 08:00:00', 300, 3.0922, 101.6498, 'Rejected', 'Insufficient resources at the facility', 1, 3, 3, 18),
('Location 19', 'Address for Location 19, Malaysia', '2025-01-19 01:00:00', '2025-01-19 06:00:00', 50, 3.0925, 101.6499, 'Approved', NULL, 1, 1, 4, 19),
('Location 20', 'Address for Location 20, Malaysia', '2025-01-20 02:00:00', '2025-01-20 07:00:00', 100, 3.0927, 101.6493, 'Pending', NULL, 1, 2, 5, 20),
('Location 21', 'Address for Location 21, Malaysia', '2025-01-21 03:00:00', '2025-01-21 08:00:00', 150, 3.0923, 101.6494, 'Rejected', 'Insufficient resources at the facility', 1, 3, 1, 21),
('Location 22', 'Address for Location 22, Malaysia', '2025-01-22 01:00:00', '2025-01-22 06:00:00', 200, 3.0926, 101.6495, 'Approved', NULL, 1, 1, 2, 22),
('Location 23', 'Address for Location 23, Malaysia', '2025-01-23 02:00:00', '2025-01-23 07:00:00', 250, 3.0924, 101.6496, 'Pending', NULL, 1, 2, 3, 23),
('Location 24', 'Address for Location 24, Malaysia', '2025-01-24 03:00:00', '2025-01-24 08:00:00', 300, 3.0922, 101.6497, 'Rejected', 'Insufficient resources at the facility', 1, 3, 4, 24),
('Location 25', 'Address for Location 25, Malaysia', '2025-01-25 01:00:00', '2025-01-25 06:00:00', 50, 3.0925, 101.6498, 'Approved', NULL, 1, 1, 5, 25),
('Location 26', 'Address for Location 26, Malaysia', '2025-01-26 02:00:00', '2025-01-26 07:00:00', 100, 3.0927, 101.6499, 'Pending', NULL, 1, 2, 1, 26),
('Location 27', 'Address for Location 27, Malaysia', '2025-01-27 03:00:00', '2025-01-27 08:00:00', 150, 3.0923, 101.6493, 'Rejected', 'Insufficient resources at the facility', 1, 3, 2, 27),
('Location 28', 'Address for Location 28, Malaysia', '2025-01-28 01:00:00', '2025-01-28 06:00:00', 200, 3.0926, 101.6494, 'Approved', NULL, 1, 1, 3, 28),
('Location 29', 'Address for Location 29, Malaysia', '2025-01-29 02:00:00', '2025-01-29 07:00:00', 250, 3.0924, 101.6495, 'Pending', NULL, 1, 2, 4, 29),
('Location 30', 'Address for Location 30, Malaysia', '2025-01-30 03:00:00', '2025-01-30 08:00:00', 300, 3.0922, 101.6496, 'Rejected', 'Insufficient resources at the facility', 1, 3, 5, 30);

INSERT INTO user_notifications (description, redirect, is_read, created_at, user_id) VALUES
('Reminder: upcoming meeting', NULL, TRUE, '2024-12-22 14:22:00', 1000),
('Dont miss this offer', NULL, FALSE, '2024-12-15 05:38:00', 1000),
('Security alert', 'events', FALSE, '2024-12-11 18:50:00', 1000),
('Special offer for you', 'events', TRUE, '2024-12-30 18:02:00', 1000),
('Event canceled', NULL, FALSE, '2024-12-16 05:42:00', 1000),
('New alert for you', 'events', FALSE, '2024-12-31 09:08:00', 1000),
('Dont miss this offer', NULL, TRUE, '2024-12-31 13:03:00', 1000),
('Important system notification', NULL, FALSE, '2024-12-15 07:20:00', 1000),
('Reminder: upcoming meeting', NULL, TRUE, '2024-12-26 07:50:00', 1000),
('Check your new achievements', NULL, TRUE, '2024-12-26 20:40:00', 1000),
('Security alert', NULL, FALSE, '2024-12-17 23:38:00', 1000),
('New message in your inbox', NULL, TRUE, '2024-12-01 12:05:00', 1000),
('Survey for feedback', NULL, FALSE, '2024-12-18 06:54:00', 1000),
('Survey for feedback', 'events', TRUE, '2024-12-15 21:24:00', 1000),
('Event is happening soon', NULL, FALSE, '2024-12-02 18:17:00', 1000),
('System update scheduled', 'events', TRUE, '2024-12-30 18:17:00', 1000),
('New alert for you', 'events', TRUE, '2024-12-03 23:37:00', 1000),
('Notification about event 1', 'events', TRUE, '2024-12-07 20:37:00', 1000);
