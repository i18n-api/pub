CREATE TABLE `mailsubMail` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`host_id` BIGINT UNSIGNED NOT NULL,`mail` VARBINARY(255) NOT NULL,`lang` SMALLINT UNSIGNED NOT NULL DEFAULT 0,PRIMARY KEY (`id`),UNIQUE KEY `host_id` (`host_id`,`mail`)
);