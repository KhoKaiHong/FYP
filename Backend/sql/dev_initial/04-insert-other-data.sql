-- Insert data into users (10 users added)
INSERT INTO users (ic_number, password, name, email, phone_number, blood_type, eligibility, state_id, district_id) VALUES
('850101-10-5001', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Ahmad bin Ali', 'ahmad.ali@example.com', '+6016-1234567', 'O+', 'Eligible', 1, 1),
('921120-05-5203', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Suresh Kumar', 'suresh.kumar@example.com', '+6016-5551212', 'A+', 'Eligible', 12, 4),
('800722-03-5305', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'David Wong', 'david.wong@example.com', '+6016-4455667', 'O-', 'Eligible', 10, 7), 
('900605-07-5607', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Ramesh Rao', 'ramesh.rao@example.com', '+6016-3344556', 'B+', 'Eligible', 7, 8),
('880402-09-5809', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Tan Ah Kau', 'ah.kau@example.com', '+6016-2233445', 'O+', 'Eligible', 2, 3), 
('961015-06-5910', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Joseph ak Luyoh', 'joseph.luyoh@example.com', '+6016-8899001', 'A+', 'Eligible', 6, 4), 
('030310-12-5104', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Dayang Nurul', 'dayang.nurul@example.com', '+6016-1122334', 'AB+', 'Ineligible', 11, 14), 
('750918-02-5406', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Siti Aminah', 'siti.aminah@example.com', '+6016-7788990', 'A-', 'Ineligible', 3, 5), 
('721228-11-5708', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Noraini binti Omar', 'noraini.omar@example.com', '+6016-6677889', 'AB-', 'Ineligible', 5, 2),
('780515-08-5502', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Lim Mei Ling', 'mei.ling@example.com', '+6016-9876543', 'B-', 'Ineligible - Condition', 9, 5);

-- Insert data into event_organisers (5 organisers added)
INSERT INTO event_organisers (email, password, name, phone_number) VALUES
('st.john.ambulans@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'St. John Ambulans Malaysia', '+6018-1112223'),
('malaysian.red.crescent@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Malaysian Red Crescent Society (MRCS)', '+6018-3334445'),
('mercy.malaysia@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Mercy Malaysia', '+6018-5556667'),
('world.vision.malaysia@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'World Vision Malaysia', '+6018-7778889'),
('unicef.malaysia@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'UNICEF Malaysia', '+6018-9990001');

-- Insert data into admins (5 admins added)
INSERT INTO admins (email, password, name) VALUES
('farid@admin.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Farid bin Rahman'),
('siew.ling@admin.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Siew Ling Tan'),
('arun@admin.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Arun Kumar Subramaniam'),
('noraidah@admin.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Noraidah bt. Abdullah'),
('vincent@admin.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Vincent Raj');

-- Insert data into blood_donation_events (19 added)
INSERT INTO blood_donation_events (location, address, start_time, end_time, max_attendees, latitude, longitude, facility_id, state_id, district_id, organiser_id) VALUES
-- Past blood donation events
('AEON Mall Tebrau City', '1, Jalan Desa Tebrau, Taman Desa Tebrau, 81100 Johor Bahru, Johor Darul Ta''zim', '2024-11-02 01:00:00', '2024-11-02 06:00:00', 50, 1.5504988882483934, 103.79546007121144, 1, 1, 2, 1),
('Aman Central', 'Aman Central, 1, Darul Aman Hwy, Kampung Lubok Peringgi, 05100 Alor Setar, Kedah', '2024-11-05 03:00:00', '2024-11-05 08:00:00', 100, 6.126557473647973, 100.36635251644108, 3, 2, (SELECT id FROM districts WHERE name = 'Kota Setar'), 3),
('Central Square', '4th Floor No, 4.09.01, 23, Jalan Kampung Baru, 08000 Sungai Petani, Kedah', '2024-11-12 03:00:00', '2024-11-12 09:00:00', 200, 5.636985974957746, 100.48850811177923, 3, 2, (SELECT id FROM districts WHERE name = 'Kuala Muda'), 4),
('KB Mall', '1, Jalan Hamzah, Bandar Kota Bharu, 15050 Kota Bharu, Kelantan', '2024-11-20 01:00:00', '2024-11-20 06:00:00', 150, 6.117535241114517, 102.24020087379189, 4, 3, (SELECT id FROM districts WHERE name = 'Kota Bharu'), 5),
('Dataran Pahlawan Melaka Megamall', 'Jln Merdeka, Banda Hilir, 75000 Melaka', '2024-11-26 02:00:00', '2024-11-26 09:00:00', 300, 2.190001569760413, 102.25044895653002, 5, 4, (SELECT id FROM districts WHERE name = 'Melaka Tengah'), 2),
('Mahkota Parade', 'Lot B-02, Mahkota Parade, 1, Jln Merdeka, Taman Costa Mahkota, 75000 Melaka', '2024-11-29 01:00:00', '2024-11-29 07:00:00', 250, 2.189337789123672, 102.24980078669556, 5, 4, (SELECT id FROM districts WHERE name = 'Melaka Tengah'), 3),
('Palm Mall', 'Palm Mall, Level 4, Jalan Sungai Ujong, Kemayan Square, 70200 Seremban, Negeri Sembilan', '2024-12-02 02:00:00', '2024-12-02 08:00:00', 100, 2.7200830958712503, 101.92291014211008, 6, 5, (SELECT id FROM districts WHERE name = 'Seremban'), 1),
('East Coast Mall', 'Jalan Putra Square 6, Putra Square, 25200 Kuantan, Pahang', '2024-12-09 03:00:00', '2024-12-09 07:00:00', 150, 3.8185819301644415, 103.32625816792614, 7, 6, (SELECT id FROM districts WHERE name = 'Kuantan'), 2),
('Ipoh Parade', '105, Jalan Sultan Abdul Jalil, Pusat Perdagangan Greentown, 30450 Ipoh, Perak', '2024-12-15 02:00:00', '2024-12-15 07:00:00', 250, 4.595751513218196, 101.08984128465725, 11, 7, (SELECT id FROM districts WHERE name = 'Kinta'), 3),
('1st Avenue Mall', '182, Jalan Magazine, 10300 George Town, Pulau Pinang', '2024-12-28 02:00:00', '2024-12-28 06:00:00', 50, 5.413053410743275, 100.33116965721827, 10, 9, (SELECT id FROM districts WHERE name = 'Timur Laut'), 1),
('Queensbay Mall', 'Queensbay Mall, 100, Persiaran Bayan Indah, 11900 Bayan Lepas, Pulau Pinang', '2025-01-05 02:00:00', '2025-01-05 07:00:00', 200, 5.333183408845015, 100.3066133670719, 10, 9, (SELECT id FROM districts WHERE name = 'Barat Daya'), 4),
-- Upcoming blood donation events
('Imago Shopping Mall', 'KK Times Square, Phase 2, Off Coastal Highway, 88100 Kota Kinabalu, Sabah', '2025-01-12 03:00:00', '2025-01-12 09:00:00', 300, 5.970860788846937, 116.06635592386823, 16, 10, (SELECT id FROM districts WHERE name = 'Kota Kinabalu'), 2),
('Vivacity Megamall', 'Q112B, 93350 Kuching, Sarawak', '2025-01-19 01:00:00', '2025-01-19 06:00:00', 100, 1.8864013570027496, 110.44913508635773, 19, 11, (SELECT id FROM districts WHERE name = 'Kuching'), 1),
('1 Utama Shopping Centre', '1, Lebuh Bandar Utama, Bandar Utama, 47800 Petaling Jaya, Selangor', '2025-01-23 03:00:00', '2025-01-23 08:00:00', 200, 3.148127466123984, 101.61646193049211, 14, 12, (SELECT id FROM districts WHERE name = 'Petaling'), 3),
('Sunway Pyramid', '3, Jalan PJS 11/15, Bandar Sunway, 47500 Petaling Jaya, Selangor', '2025-01-28 01:00:00', '2025-01-28 09:00:00', 100, 3.0739684691784834, 101.60737446217043, 14, 12, (SELECT id FROM districts WHERE name = 'Petaling'), 5),
('Mesra Mall', 'Mesra Mall, Lot 6490, Jalan Kemaman - Dungun, Kampung Baru, 24200 Kemasik, Terengganu', '2025-02-03 03:00:00', '2025-02-03 06:00:00', 300, 4.440697264195204, 103.44849511840275, 15, 13, (SELECT id FROM districts WHERE name = 'Dungun'), 1),
('Pavilion Kuala Lumpur', '168, Jln Bukit Bintang, Bukit Bintang, 55100 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '2025-02-10 03:00:00', '2025-02-10 07:00:00', 150, 3.148998022026934, 101.71340731862986, 22, 14, (SELECT id FROM districts WHERE name = 'W.P. Kuala Lumpur'), 2),
('Financial Park Labuan Complex', 'Jln Merdeka, Financial Park, 87000 Labuan, Wilayah Persekutuan Labuan', '2025-02-17 02:00:00', '2025-02-17 09:00:00', 100, 5.276586556815842, 115.24945783706148, 22, 15, (SELECT id FROM districts WHERE name = 'W.P. Labuan'), 3),
('Alamanda Shopping Centre', 'Jalan Alamanda, Presint 1, 62000 Putrajaya, Wilayah Persekutuan Putrajaya', '2025-02-25 01:00:00', '2025-02-25 08:00:00', 200, 2.939551932738423, 101.71061614346809, 22, 16, (SELECT id FROM districts WHERE name = 'W.P. Putrajaya'), 4); 

-- Insert data into registrations (10 added)
INSERT INTO registrations (status, event_id, user_id, registered_at) VALUES
('Attended', 1, 1006, '2024-10-10 03:28:38'),
('Attended', 2, 1007, '2024-10-30 08:10:20'),
('Attended', 3, 1008, '2024-11-02 10:56:09'),
('Absent', 4, 1003, '2024-11-05 03:46:18'),
('Absent', 5, 1004, '2024-11-20 12:10:34'),
('Registered', 12, 1000, '2024-12-02 07:12:23'),
('Registered', 13, 1001, '2024-12-12 14:14:57'),
('Registered', 14, 1002, '2024-12-14 12:03:05'),
('Registered', 15, 1003, '2024-12-20 13:23:12'),
('Registered', 16, 1004, '2024-12-28 10:05:47');

-- Insert data into donation_history (3 added)
INSERT INTO donation_history (user_id, event_id, created_at) VALUES
(1006, 1, '2024-11-02 02:34:10'),
(1007, 2, '2024-11-05 06:23:05'),
(1008, 3, '2024-11-12 08:12:00');

-- Insert new blood donation events requests (24 added)
INSERT INTO new_blood_donation_events_requests (
    location, address, start_time, end_time, max_attendees,
    latitude, longitude, status, rejection_reason, facility_id, state_id, district_id, organiser_id
) VALUES
-- Approved requests
-- Past blood donation events
('AEON Mall Tebrau City', '1, Jalan Desa Tebrau, Taman Desa Tebrau, 81100 Johor Bahru, Johor Darul Ta''zim', '2024-11-02 01:00:00', '2024-11-02 06:00:00', 50, 1.5504988882483934, 103.79546007121144, 'Approved', NULL, 1, 1, 2, 1),
('Aman Central', 'Aman Central, 1, Darul Aman Hwy, Kampung Lubok Peringgi, 05100 Alor Setar, Kedah', '2024-11-05 03:00:00', '2024-11-05 08:00:00', 100, 6.126557473647973, 100.36635251644108, 'Approved', NULL, 3, 2, (SELECT id FROM districts WHERE name = 'Kota Setar'), 3),
('Central Square', '4th Floor No, 4.09.01, 23, Jalan Kampung Baru, 08000 Sungai Petani, Kedah', '2024-11-12 03:00:00', '2024-11-12 09:00:00', 200, 5.636985974957746, 100.48850811177923, 'Approved', NULL, 3, 2, (SELECT id FROM districts WHERE name = 'Kuala Muda'), 4),
('KB Mall', '1, Jalan Hamzah, Bandar Kota Bharu, 15050 Kota Bharu, Kelantan', '2024-11-20 01:00:00', '2024-11-20 06:00:00', 150, 6.117535241114517, 102.24020087379189, 'Approved', NULL, 4, 3, (SELECT id FROM districts WHERE name = 'Kota Bharu'), 5),
('Dataran Pahlawan Melaka Megamall', 'Jln Merdeka, Banda Hilir, 75000 Melaka', '2024-11-26 02:00:00', '2024-11-26 09:00:00', 300, 2.190001569760413, 102.25044895653002, 'Approved', NULL, 5, 4, (SELECT id FROM districts WHERE name = 'Melaka Tengah'), 2),
('Mahkota Parade', 'Lot B-02, Mahkota Parade, 1, Jln Merdeka, Taman Costa Mahkota, 75000 Melaka', '2024-11-29 01:00:00', '2024-11-29 07:00:00', 250, 2.189337789123672, 102.24980078669556, 'Approved', NULL, 5, 4, (SELECT id FROM districts WHERE name = 'Melaka Tengah'), 3),
('Palm Mall', 'Palm Mall, Level 4, Jalan Sungai Ujong, Kemayan Square, 70200 Seremban, Negeri Sembilan', '2024-12-02 02:00:00', '2024-12-02 08:00:00', 100, 2.7200830958712503, 101.92291014211008, 'Approved', NULL, 6, 5, (SELECT id FROM districts WHERE name = 'Seremban'), 1),
('East Coast Mall', 'Jalan Putra Square 6, Putra Square, 25200 Kuantan, Pahang', '2024-12-09 03:00:00', '2024-12-09 07:00:00', 150, 3.8185819301644415, 103.32625816792614, 'Approved', NULL, 7, 6, (SELECT id FROM districts WHERE name = 'Kuantan'), 2),
('Ipoh Parade', '105, Jalan Sultan Abdul Jalil, Pusat Perdagangan Greentown, 30450 Ipoh, Perak', '2024-12-15 02:00:00', '2024-12-15 07:00:00', 250, 4.595751513218196, 101.08984128465725, 'Approved', NULL, 11, 7, (SELECT id FROM districts WHERE name = 'Kinta'), 3),
('1st Avenue Mall', '182, Jalan Magazine, 10300 George Town, Pulau Pinang', '2024-12-28 02:00:00', '2024-12-28 06:00:00', 50, 5.413053410743275, 100.33116965721827, 'Approved', NULL, 10, 9, (SELECT id FROM districts WHERE name = 'Timur Laut'), 1),
('Queensbay Mall', 'Queensbay Mall, 100, Persiaran Bayan Indah, 11900 Bayan Lepas, Pulau Pinang', '2025-01-05 02:00:00', '2025-01-05 07:00:00', 200, 5.333183408845015, 100.3066133670719, 'Approved', NULL, 10, 9, (SELECT id FROM districts WHERE name = 'Barat Daya'), 4),
-- Upcoming blood donation events
('Imago Shopping Mall', 'KK Times Square, Phase 2, Off Coastal Highway, 88100 Kota Kinabalu, Sabah', '2025-01-12 03:00:00', '2025-01-12 09:00:00', 300, 5.970860788846937, 116.06635592386823, 'Approved', NULL, 16, 10, (SELECT id FROM districts WHERE name = 'Kota Kinabalu'), 2),
('Vivacity Megamall', 'Q112B, 93350 Kuching, Sarawak', '2025-01-19 01:00:00', '2025-01-19 06:00:00', 100, 1.8864013570027496, 110.44913508635773, 'Approved', NULL, 19, 11, (SELECT id FROM districts WHERE name = 'Kuching'), 1),
('1 Utama Shopping Centre', '1, Lebuh Bandar Utama, Bandar Utama, 47800 Petaling Jaya, Selangor', '2025-01-23 03:00:00', '2025-01-23 08:00:00', 200, 3.148127466123984, 101.61646193049211, 'Approved', NULL, 14, 12, (SELECT id FROM districts WHERE name = 'Petaling'), 3),
('Sunway Pyramid', '3, Jalan PJS 11/15, Bandar Sunway, 47500 Petaling Jaya, Selangor', '2025-01-28 01:00:00', '2025-01-28 09:00:00', 100, 3.0739684691784834, 101.60737446217043, 'Approved', NULL, 14, 12, (SELECT id FROM districts WHERE name = 'Petaling'), 5),
('Mesra Mall', 'Mesra Mall, Lot 6490, Jalan Kemaman - Dungun, Kampung Baru, 24200 Kemasik, Terengganu', '2025-02-03 03:00:00', '2025-02-03 06:00:00', 300, 4.440697264195204, 103.44849511840275, 'Approved', NULL, 15, 13, (SELECT id FROM districts WHERE name = 'Dungun'), 1),
('Pavilion Kuala Lumpur', '168, Jln Bukit Bintang, Bukit Bintang, 55100 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '2025-02-10 03:00:00', '2025-02-10 07:00:00', 150, 3.148998022026934, 101.71340731862986, 'Approved', NULL, 22, 14, (SELECT id FROM districts WHERE name = 'W.P. Kuala Lumpur'), 2),
('Financial Park Labuan Complex', 'Jln Merdeka, Financial Park, 87000 Labuan, Wilayah Persekutuan Labuan', '2025-02-17 02:00:00', '2025-02-17 09:00:00', 100, 5.276586556815842, 115.24945783706148, 'Approved', NULL, 22, 15, (SELECT id FROM districts WHERE name = 'W.P. Labuan'), 3),
('Alamanda Shopping Centre', 'Jalan Alamanda, Presint 1, 62000 Putrajaya, Wilayah Persekutuan Putrajaya', '2025-02-25 01:00:00', '2025-02-25 08:00:00', 200, 2.939551932738423, 101.71061614346809, 'Approved', NULL, 22, 16, (SELECT id FROM districts WHERE name = 'W.P. Putrajaya'), 4),
-- Rejected requests
('Mid Valley Megamall', 'Lingkaran Syed Putra, Mid Valley City, 59200 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '2025-01-23 01:00:00', '2025-01-23 07:00:00', 50, 3.1177141222234273, 101.67739991060756, 'Rejected', 'Insufficient manpower to dispatch at the facility', 22, 14, (SELECT id FROM districts WHERE name = 'W.P. Kuala Lumpur'), 1),
('Berjaya Times Square', 'No. 1, Jalan Imbi, 55100 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '2025-01-26 02:00:00', '2025-01-26 09:00:00', 100, 3.1423269411587826, 101.71059408876324, 'Rejected', 'No transportation options available at the time', 22, 14, (SELECT id FROM districts WHERE name = 'W.P. Kuala Lumpur'), 3),
('Suria KLCC', '241, Petronas Twin Tower, Kuala Lumpur City Centre, 50088 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '2025-02-03 03:00:00', '2025-02-03 06:00:00', 250, 3.1582885400344383, 101.71220350175281, 'Rejected', 'Location does not meet requirements', 22, 14, (SELECT id FROM districts WHERE name = 'W.P. Kuala Lumpur'), 5),
-- Pending requests
('NU Sentral', 'Nu Sentral Shopping Mall, Jalan Tun Sambanthan, City Centre, 50470 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '2025-02-14 01:00:00', '2025-02-14 07:00:00', 50, 3.1330861476540046, 101.68742603204397, 'Pending', NULL, 22, 14, (SELECT id FROM districts WHERE name = 'W.P. Kuala Lumpur'), 2),
('Plaza Low Yat', '7, Jalan Bintang, Off Jalan Bukit Bintang, Central, Bukit Bintang, 55100 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '2025-02-26 02:00:00', '2025-02-26 09:00:00', 100, 3.1441822028345365, 101.70996143106056, 'Pending', NULL, 22, 14, (SELECT id FROM districts WHERE name = 'W.P. Kuala Lumpur'), 4);

