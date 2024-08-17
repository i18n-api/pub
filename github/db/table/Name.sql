CREATE TABLE `githubName` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`github_user_id` BIGINT UNSIGNED NOT NULL,`name` VARBINARY(255) DEFAULT NULL,PRIMARY KEY (`id`),UNIQUE KEY `githubUserName` (`github_user_id`,`name`)
);