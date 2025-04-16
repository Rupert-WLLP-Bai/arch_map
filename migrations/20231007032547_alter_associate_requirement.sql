-- Add migration script here
ALTER TABLE Documents ALTER COLUMN associate_requirement SET DEFAULT '00000000-0000-0000-0000-000000000000';
UPDATE Documents SET associate_requirement = '00000000-0000-0000-0000-000000000000' WHERE associate_requirement IS NULL;
ALTER TABLE Documents ALTER COLUMN associate_requirement SET NOT NULL;
