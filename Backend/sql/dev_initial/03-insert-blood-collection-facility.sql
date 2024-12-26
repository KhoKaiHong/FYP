-- Insert blood collection facilities (22 added)
INSERT INTO "blood_collection_facilities" (name, email, password, address, phone_number, state_id) VALUES
('Hospital Sultanah Aminah', 'hospital.sultanah.aminah@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan, Persiaran Abu Bakar Sultan, 80100 Johor Bahru, Johor Darul Ta''zim', '+6012-8621009', (SELECT id FROM states WHERE name = 'Johor')),
('Hospital Sultanah Nora Ismail', 'hospital.sultanah.nora.ismail@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Korma, Taman Soga, 83000 Batu Pahat, Johor Darul Ta''zim', '+6012-9048686', (SELECT id FROM states WHERE name = 'Johor')),
('Hospital Sultanah Bahiyah', 'hospital.sultanah.bahiyah@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Km 6, Jln Langgar, Bandar, 05460 Alor Setar, Kedah', '+6012-4114816', (SELECT id FROM states WHERE name = 'Kedah')),
('Hospital Raja Perempuan Zainab II', 'hospital.raja.perempuan.zainab.ii@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Bandar Kota Bharu, 15586 Kota Bharu, Kelantan', '+6012-1326050', (SELECT id FROM states WHERE name = 'Kelantan')),
('Hospital Melaka', 'hospital.melaka@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Mufti Haji Khalil, 75400 Melaka', '+6012-2332154', (SELECT id FROM states WHERE name = 'Melaka')),
('Hospital Tuanku Ja''afar', 'hospital.tuanku.jaafar@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Rasah, Bukit Rasah, 70300 Seremban, Negeri Sembilan', '+6012-9407402', (SELECT id FROM states WHERE name = 'Negeri Sembilan')),
('Hospital Tengku Ampuan Afzan', 'hospital.tengku.ampuan.afzan@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Tanah Putih, 25100 Kuantan, Pahang', '+6012-9795894', (SELECT id FROM states WHERE name = 'Pahang')),
('Hospital Sultan Haji Ahmad Shah', 'hospital.sultan.haji.ahmad.shah@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Maran, Taman Harapan, 28000 Temerloh, Pahang', '+6012-1673853', (SELECT id FROM states WHERE name = 'Pahang')),
('Hospital Seberang Jaya', 'hospital.seberang.jaya@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jln Tun Hussein Onn, Seberang Jaya, 13700 Perai, Pulau Pinang', '+6012-5975152', (SELECT id FROM states WHERE name = 'Pulau Pinang')),
('Hospital Pulau Pinang', 'hospital.pulau.pinang@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Residensi, 10450 George Town, Pulau Pinang', '+6012-5011793', (SELECT id FROM states WHERE name = 'Pulau Pinang')),
('Hospital Raja Permaisuri Bainun', 'hospital.raja.permaisuri.bainun@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Raja Ashman Shah, 30450 Ipoh, Perak', '+6012-2423874', (SELECT id FROM states WHERE name = 'Perak')),
('Hospital Taiping', 'hospital.taiping@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Taming Sari, 34000 Taiping, Perak', '+6012-3735581', (SELECT id FROM states WHERE name = 'Perak')),
('Hospital Seri Manjung', 'hospital.seri.manjung@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', '32040 Seri Manjung, Perak', '+6012-5306522', (SELECT id FROM states WHERE name = 'Perak')),
('Hospital Tengku Ampuan Rahimah', 'hospital.tengku.ampuan.rahimah@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Langat, 41200 Klang, Selangor', '+6012-3479056', (SELECT id FROM states WHERE name = 'Selangor')),
('Hospital Sultanah Nur Zahirah', 'hospital.sultanah.nur.zahirah@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', '84GX+RH, Jalan Sultan Mahmud, 20400 Kuala Terengganu, Terengganu', '+6012-9849753', (SELECT id FROM states WHERE name = 'Terengganu')),
('Hospital Queen Elizabeth II', 'hospital.queen.elizabeth.ii@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Lorong Bersatu, Off, Jalan Damai, Luyang Commercial Centre, 88300 Kota Kinabalu, Sabah', '+6012-7708305', (SELECT id FROM states WHERE name = 'Sabah')),
('Hospital Duchess Of Kent', 'hospital.duchess.of.kent@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'KM 3.2, Jalan Utara, 90000 Sandakan, Sabah', '+6012-7305322', (SELECT id FROM states WHERE name = 'Sabah')),
('Hospital Tawau', 'hospital.tawau@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Peti Surat 67, 91000 Tawau, Sabah', '+6012-9173125', (SELECT id FROM states WHERE name = 'Sabah')),
('Hospital Umum Sarawak', 'hospital.umum.sarawak@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jalan Hospital, 93586 Kuching, Sarawak', '+6012-3555410', (SELECT id FROM states WHERE name = 'Sarawak')),
('Hospital Miri', 'hospital.miri@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Q353, 98000 Miri, Sarawak', '+6012-8306204', (SELECT id FROM states WHERE name = 'Sarawak')),
('Hospital Sibu', 'hospital.sibu@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Batu 5 1/2, Jalan Ulu Oya, Pekan Sibu, 96000 Sibu, Sarawak', '+6012-3805338', (SELECT id FROM states WHERE name = 'Sarawak')),
('Pusat Darah Negara', 'pusat.darah.negara@example.com', '$argon2id$v=19$m=16,t=2,p=1$a1hTTzVvYXFaajBEemV6bA$1UwkkC3BQXNZmBTBgzVo0A', 'Jln Tun Razak, Titiwangsa, 50400 Kuala Lumpur, Wilayah Persekutuan Kuala Lumpur', '+6012-8755197 ', (SELECT id FROM states WHERE name = 'W.P. Kuala Lumpur'));