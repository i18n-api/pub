CREATE TABLE `githubX` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`github_user_id` BIGINT UNSIGNED NOT NULL,`name` VARBINARY(255) DEFAULT NULL,PRIMARY KEY (`id`),UNIQUE KEY `githubXName` (`name`,`github_user_id`)
);