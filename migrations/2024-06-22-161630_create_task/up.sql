-- Your SQL goes here
CREATE TABLE `task`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`precedence` INTEGER NOT NULL,
	`status_id` INTEGER NOT NULL,
	FOREIGN KEY (`status_id`) REFERENCES `status`(`id`)
);

