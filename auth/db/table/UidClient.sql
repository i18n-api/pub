CREATE TABLE `authUidClient` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`uid` BIGINT UNSIGNED NOT NULL,`client` BIGINT UNSIGNED NOT NULL,`state` TINYINT UNSIGNED NOT NULL,`cts` BIGINT UNSIGNED NOT NULL,`lastSignInId` BIGINT UNSIGNED NOT NULL DEFAULT 0,PRIMARY KEY (`id`),UNIQUE KEY `uid` (`uid`,`client`)
);