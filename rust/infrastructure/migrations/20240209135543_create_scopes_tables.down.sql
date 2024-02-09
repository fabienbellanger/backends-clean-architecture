-- Add down migration script here
ALTER TABLE `users_scopes` DROP FOREIGN KEY `fk_users_scopes_user_id`;
ALTER TABLE `users_scopes` DROP FOREIGN KEY `fk_users_scopes_scope_id`;

DROP TABLE IF EXISTS `users_scopes`;
DROP TABLE IF EXISTS `scopes`;
