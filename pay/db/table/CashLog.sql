CREATE TABLE `payCashLog` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`ts` BIGINT UNSIGNED NOT NULL,`n` BIGINT NOT NULL COMMENT '余额快照',`diff` BIGINT NOT NULL,`bill_id` BIGINT UNSIGNED NOT NULL,PRIMARY KEY (`id`)
);