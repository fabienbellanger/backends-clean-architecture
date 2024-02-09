-- Add up migration script here
CREATE TABLE IF NOT EXISTS `scopes`
(
    `id`         VARCHAR(127) NOT NULL,
    `created_at` DATETIME(3)  NOT NULL,
    `deleted_at` DATETIME(3),
    PRIMARY KEY (`id`),
    INDEX `idx_scopes_deleted_at` (`deleted_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE IF NOT EXISTS `users_scopes`
(
    `user_id`  VARCHAR(36)  NOT NULL,
    `scope_id` VARCHAR(127) NOT NULL,
    PRIMARY KEY (`user_id`, `scope_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

ALTER TABLE `users_scopes`
    ADD CONSTRAINT `fk_users_scopes_user_id`
        FOREIGN KEY (`user_id`)
            REFERENCES `users` (`id`) ON DELETE CASCADE;

ALTER TABLE `users_scopes`
    ADD CONSTRAINT `fk_users_scopes_scope_id`
        FOREIGN KEY (`scope_id`)
            REFERENCES `scopes` (`id`) ON DELETE CASCADE;
