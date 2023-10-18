-- Add down migration script here
ALTER TABLE `refresh_tokens` DROP FOREIGN KEY `fk_refresh_tokens_user_id`;

DROP TABLE IF EXISTS `refresh_tokens`;
