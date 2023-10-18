-- Add up migration script here
CREATE TABLE IF NOT EXISTS `refresh_tokens` (
    `refresh_token` VARCHAR(36) NOT NULL,
    `user_id` VARCHAR(36) NOT NULL,
    `access_token` TEXT NOT NULL,
    `expired_at` DATETIME(3) NOT NULL,
    PRIMARY KEY (`refresh_token`),
    INDEX `idx_refresh_tokens_user_id` (`user_id`),
    INDEX `idx_refresh_tokens_access_token` (`access_token`),
    INDEX `idx_refresh_tokens_expired_at` (`expired_at`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;

ALTER TABLE `refresh_tokens`
ADD CONSTRAINT `fk_refresh_tokens_user_id`
FOREIGN KEY (`user_id`)
REFERENCES `users`(`id`) ON DELETE CASCADE;
