CREATE TABLE `authPasswdLog` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`uid` BIGINT UNSIGNED NOT NULL,`hash` BINARY(16) NOT NULL,`ts` BIGINT UNSIGNED NOT NULL,`dts` BIGINT UNSIGNED NOT NULL,PRIMARY KEY (`id`),UNIQUE KEY `uidTs` (`uid`,`ts`)
);