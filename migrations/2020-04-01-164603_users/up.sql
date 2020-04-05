-- Your SQL goes here
CREATE TABLE users (
  `id` INT(11) PRIMARY KEY AUTO_INCREMENT,
  `created_at` DATETIME,
  `updated_at` DATETIME,
  `name` VARCHAR(60) NOT NULL,
  `email` VARCHAR(100) NOT NULL,
  `latitude` DECIMAL(10, 8),
  `longitude` DECIMAL(11, 8),
  `language` VARCHAR(4),
  `country` VARCHAR(60),
  `region` VARCHAR(60),
  `city` VARCHAR(60),
  `gender` VARCHAR(60),
  `age` INT(11)
)