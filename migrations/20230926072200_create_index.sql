-- Add migration script here
-- 在Documents表上为name字段创建索引
CREATE INDEX idx_documents_name ON Documents(name);

-- 在TechL1表上为name字段创建索引
CREATE INDEX idx_techl1_name ON TechL1(name);

-- 在TechL2表上为name字段创建索引
CREATE INDEX idx_techl2_name ON TechL2(name);

-- 在SystemL1表上为name字段创建索引
CREATE INDEX idx_systeml1_name ON SystemL1(name);

-- 在SystemL2表上为name字段创建索引
CREATE INDEX idx_systeml2_name ON SystemL2(name);

-- 在MFL1表上为name字段创建索引
CREATE INDEX idx_mfl1_name ON MFL1(name);

-- 在MFL2表上为name字段创建索引
CREATE INDEX idx_mfl2_name ON MFL2(name);

-- 在ProjectL1表上为name字段创建索引
CREATE INDEX idx_projectl1_name ON ProjectL1(name);

-- 在ProjectL2表上为name字段创建索引
CREATE INDEX idx_projectl2_name ON ProjectL2(name);

