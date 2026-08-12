-- Character names cross the custom client protocol as UTF-8. Keep the
-- existing field width and data while making the storage encoding compatible.
ALTER TABLE `characters`
  MODIFY `name` varchar(13)
  CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci
  NOT NULL DEFAULT '';
