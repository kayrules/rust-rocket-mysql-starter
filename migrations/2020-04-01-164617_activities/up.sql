-- Your SQL goes here
CREATE TABLE activities (
  `id` INT(11) PRIMARY KEY AUTO_INCREMENT,
  `created_at` DATETIME,
  `updated_at` DATETIME,
  `activity` VARCHAR(30) NOT NULL,
  `status` TINYINT(2) NOT NULL DEFAULT '0',
  `user_1` INT(11),
  `user_2` INT(11),
  `identifier_1` INT(11),
  `identifier_2` INT(11),
  `param_1` VARCHAR(100),
  `param_2` VARCHAR(100),
  `param_3` VARCHAR(100)
)