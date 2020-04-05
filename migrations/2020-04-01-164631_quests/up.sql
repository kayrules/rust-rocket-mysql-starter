-- Your SQL goes here
CREATE TABLE quests (
  `id` INT(11) PRIMARY KEY AUTO_INCREMENT,
  `created_at` DATETIME,
  `updated_at` DATETIME,
  `quest` VARCHAR(255) NOT NULL,
  `status` TINYINT(2) NOT NULL DEFAULT '0',
  `points` INT(11) NOT NULL DEFAULT '0',
  `level` VARCHAR(20) NOT NULL,
  `icon_url` VARCHAR(255),
  `activity` VARCHAR(30) NOT NULL,
  `activity_type` VARCHAR(30),
  `activity_count` INT(11) NOT NULL DEFAULT '0'
)