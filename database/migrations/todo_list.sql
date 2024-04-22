-- 待做列表
CREATE TABLE `todo_list` (
    `id` BIGINT AUTO_INCREMENT,
    `description` VARCHAR(50) NOT NULL,
    `completed` BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
);