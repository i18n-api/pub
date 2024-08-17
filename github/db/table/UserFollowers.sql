CREATE TABLE `githubUserFollowers` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`github_user_id` BIGINT UNSIGNED NOT NULL,`n` BIGINT UNSIGNED NOT NULL,`ts` BIGINT UNSIGNED NOT NULL DEFAULT (unix_timestamp()),PRIMARY KEY (`id`),UNIQUE KEY `github_user_id` (`github_user_id`,`n`)
);