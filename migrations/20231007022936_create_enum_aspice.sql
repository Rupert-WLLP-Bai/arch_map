-- Add migration script here
CREATE TYPE ASPICE AS ENUM ('需求', '架构', '详设', '单测', '集测', '路测');

CREATE TABLE document_aspice_mapping (
    docid UUID REFERENCES Documents(id),
    aspice_step ASPICE NOT NULL,
    PRIMARY KEY (docid, aspice_step)
);
CREATE INDEX idx_docid_on_document_aspice_mapping ON document_aspice_mapping (docid);

ALTER TABLE Documents ADD COLUMN associate_requirement UUID;