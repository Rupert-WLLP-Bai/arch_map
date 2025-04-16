// 对应于 `Documents` 表
use uuid::Uuid;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub link: String,
    pub description: Option<String>,
    pub associate_requirement: Uuid,
}

// 对应于 `TechL1` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TechL1 {
    pub id: Uuid,
    pub name: String,
}

// 对应于 `TechL2` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TechL2 {
    pub id: Uuid,
    pub name: String,
    pub father_id: Uuid,
}



// 对应于 `DocumentTech` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DocumentTech {
    pub doc_id: Uuid,
    pub tech_id: Uuid,
}

// 对应于 `SystemL1` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SystemL1 {
    pub id: Uuid,
    pub name: String,
}

// 对应于 `SystemL2` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SystemL2 {
    pub id: Uuid,
    pub name: String,
    pub father_id: Uuid,
}

// 对应于 `DocumentSystem` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DocumentSystem {
    pub doc_id: Uuid,
    pub sys_id: Uuid,
}

// 对应于 `MFL1` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MfL1 {
    pub id: Uuid,
    pub name: String,
}

// 对应于 `MFL2` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MfL2 {
    pub id: Uuid,
    pub name: String,
    pub father_id: Uuid,
}

// 对应于 `DocumentMF` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DocumentMf {
    pub doc_id: Uuid,
    pub mf_id: Uuid,
}

// 对应于 `ProjectL1` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProjectL1 {
    pub id: Uuid,
    pub name: String,
}

// 对应于 `ProjectL2` 表
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProjectL2 {
    pub id: Uuid,
    pub name: String,
    pub father_id: Uuid,
}

// 对应于 `DocumentProject` 表
#[derive(Debug, Clone, sqlx::FromRow, PartialEq, PartialOrd, Eq, Ord)]
pub struct DocumentProject {
    pub doc_id: Uuid,
    pub project_id: Uuid,
}

// mapping to pg enum type: aspice
#[derive(sqlx::Type, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[sqlx(type_name = "aspice", rename_all = "lowercase")]
pub enum Aspice {
    需求,
    架构,
    详设,
    单测,
    集测,
    路测,
}

// CREATE TABLE document_aspice_mapping (
//     docid UUID REFERENCES Documents(id),
//     aspice_step ASPICE NOT NULL,
//     PRIMARY KEY (docid, aspice_step)
// );
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DocumentAspiceMapping {
    pub docid: Uuid,
    pub aspice_step: Aspice,
}

impl Document {
    pub fn new(name: String, link: String, description: Option<String>) -> Document {
        Document {
            id: Uuid::new_v4(),
            name,
            link,
            description,
            associate_requirement: Uuid::nil(),
        }
    }

    pub fn to_json_value(&self, is_related: bool) -> serde_json::Value {
        serde_json::json!({
            "id": self.id.to_string(),
            "name": self.name,
            "address": self.link,
            "description": self.description,
            "isRelated": is_related,
        })
    }
}

impl TechL1 {
    pub fn new(name: String) -> TechL1 {
        TechL1 {
            id: Uuid::new_v4(),
            name,
        }
    }
}

impl TechL2 {
    pub fn new(name: String, father_id: Uuid) -> TechL2 {
        TechL2 {
            id: Uuid::new_v4(),
            name,
            father_id,
        }
    }
}

impl DocumentTech {
    pub fn new(doc_id: Uuid, tech_id: Uuid) -> DocumentTech {
        DocumentTech {
            doc_id,
            tech_id,
        }
    }
}

impl SystemL1 {
    pub fn new(name: String) -> SystemL1 {
        SystemL1 {
            id: Uuid::new_v4(),
            name,
        }
    }
}

impl SystemL2 {
    pub fn new(name: String, father_id: Uuid) -> SystemL2 {
        SystemL2 {
            id: Uuid::new_v4(),
            name,
            father_id,
        }
    }
}

impl DocumentSystem {
    pub fn new(doc_id: Uuid, sys_id: Uuid) -> DocumentSystem {
        DocumentSystem {
            doc_id,
            sys_id,
        }
    }
}

impl MfL1 {
    pub fn new(name: String) -> MfL1 {
        MfL1 {
            id: Uuid::new_v4(),
            name,
        }
    }
}

impl MfL2 {
    pub fn new(name: String, father_id: Uuid) -> MfL2 {
        MfL2 {
            id: Uuid::new_v4(),
            name,
            father_id,
        }
    }
}

impl DocumentMf {
    pub fn new(doc_id: Uuid, mf_id: Uuid) -> DocumentMf {
        DocumentMf {
            doc_id,
            mf_id,
        }
    }
}

impl ProjectL1 {
    pub fn new(name: String) -> ProjectL1 {
        ProjectL1 {
            id: Uuid::new_v4(),
            name,
        }
    }
}

impl ProjectL2 {
    pub fn new(name: String, father_id: Uuid) -> ProjectL2 {
        ProjectL2 {
            id: Uuid::new_v4(),
            name,
            father_id,
        }
    }
}

impl DocumentProject {
    pub fn new(doc_id: Uuid, project_id: Uuid) -> DocumentProject {
        DocumentProject {
            doc_id,
            project_id,
        }
    }
}

impl Aspice {
    pub fn from_str(s: &str) -> Option<Aspice> {
        match s {
            "需求" => Some(Aspice::需求),
            "架构" => Some(Aspice::架构),
            "详设" => Some(Aspice::详设),
            "单测" => Some(Aspice::单测),
            "集测" => Some(Aspice::集测),
            "路测" => Some(Aspice::路测),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Aspice::需求 => "软件需求分析".to_string(),
            Aspice::架构 => "软件架构设计".to_string(),
            Aspice::详设 => "软件详设设计和单元构建".to_string(),
            Aspice::单测 => "软件单元验证".to_string(),
            Aspice::集测 => "软件集成和集成测试".to_string(),
            Aspice::路测 => "软件合格性测试".to_string(),
        }
    }
}

impl DocumentAspiceMapping {
    pub fn new(docid: Uuid, aspice_step: Aspice) -> DocumentAspiceMapping {
        DocumentAspiceMapping {
            docid,
            aspice_step,
        }
    }
}