CREATE TABLE `authGpu` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`v` VARBINARY(255) NOT NULL,PRIMARY KEY (`id`),UNIQUE KEY `v` (`v`)
);