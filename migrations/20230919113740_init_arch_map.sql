-- -- This file should undo anything in `up.sql`
-- DROP TABLE IF EXISTS DocumentProject;
-- DROP TABLE IF EXISTS ProjectL2;
-- DROP TABLE IF EXISTS ProjectL1;

-- DROP TABLE IF EXISTS DocumentMF;
-- DROP TABLE IF EXISTS MFL2;
-- DROP TABLE IF EXISTS MFL1;

-- DROP TABLE IF EXISTS DocumentSystem;
-- DROP TABLE IF EXISTS SystemL2;
-- DROP TABLE IF EXISTS SystemL1;

-- DROP TABLE IF EXISTS DocumentTech;
-- DROP TABLE IF EXISTS TechL2;
-- DROP TABLE IF EXISTS TechL1;

-- DROP TABLE IF EXISTS Documents;


-- Add migration script here
-- Document Table
CREATE TABLE Documents (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    link VARCHAR(255) NOT NULL,
    description VARCHAR(255)
);

-- Tech Level 1 and 2
CREATE TABLE TechL1 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE TechL2 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    fatherid UUID REFERENCES TechL1(id)
);

-- Document-Tech association
CREATE TABLE DocumentTech (
    docid UUID REFERENCES Documents(id),
    Techid UUID REFERENCES TechL2(id),
    PRIMARY KEY (docid, Techid)
);

-- System Level 1 and 2
CREATE TABLE SystemL1 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE SystemL2 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    fatherid UUID REFERENCES SystemL1(id)
);

-- Document-System association
CREATE TABLE DocumentSystem (
    docid UUID REFERENCES Documents(id),
    sysid UUID REFERENCES SystemL2(id),
    PRIMARY KEY (docid, sysid)
);

-- MF Software Level 1 and 2
CREATE TABLE MFL1 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE MFL2 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    fatherid UUID REFERENCES MFL1(id)
);

-- Document-MF Software association
CREATE TABLE DocumentMF (
    docid UUID REFERENCES Documents(id),
    MFid UUID REFERENCES MFL2(id),
    PRIMARY KEY (docid, MFid)
);

-- Project Level 1 and 2
CREATE TABLE ProjectL1 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE ProjectL2 (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    fatherid UUID REFERENCES ProjectL1(id)
);

-- Document-Project association
CREATE TABLE DocumentProject (
    docid UUID REFERENCES Documents(id),
    projectid UUID REFERENCES ProjectL2(id),
    PRIMARY KEY (docid, projectid)
);
