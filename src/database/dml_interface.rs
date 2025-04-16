use super::models::*;
use anyhow::Context;
use async_once::AsyncOnce;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
//use crate::etl::transform_load::InternalData;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

lazy_static! {
    static ref DB_POOL: AsyncOnce<anyhow::Result<PgPool>> = AsyncOnce::new(async {
        let database_url = dotenvy::var("DATABASE_URL")
        // The error from `var()` doesn't mention the environment variable.
        .context("DATABASE_URL must be set")?;

        let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("failed to connect to DATABASE_URL")?;

        Ok(db)
    });
}

pub async fn get_db_pool() -> anyhow::Result<sqlx::PgPool> {
    let pool_ref = DB_POOL.get().await;
    match pool_ref {
        Ok(pool) => Ok(pool.clone()), // Clone the PgPool
        Err(err) => Err(anyhow::anyhow!(err)),
    }
}


pub async fn insert_documents(pool: &sqlx::PgPool, docs: Vec<Document>) -> anyhow::Result<()> {
    for doc in docs {
        //dbg!(&doc);
        sqlx::query!(
            r#"
            INSERT INTO documents (id, name, link, description, associate_requirement)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            doc.id,
            doc.name,
            doc.link,
            doc.description,
            doc.associate_requirement,
        )
        .execute(pool)
        .await
        .context("failed to insert documents")?;
    }

    Ok(())
}

pub async fn insert_tech_l1(pool: &sqlx::PgPool, techs: Vec<TechL1>) -> anyhow::Result<()> {
    for tech in techs {
        sqlx::query!(
            r#"
            INSERT INTO techl1 (id, name)
            VALUES ($1, $2)
            "#,
            tech.id,
            tech.name,
        )
        .execute(pool)
        .await
        .context("failed to insert tech l1")?;
    }

    Ok(())
}

pub async fn insert_tech_l2(pool: &sqlx::PgPool, techs: Vec<TechL2>) -> anyhow::Result<()> {
    for tech in techs {
        sqlx::query!(
            r#"
            INSERT INTO techl2 (id, name, fatherid)
            VALUES ($1, $2, $3)
            "#,
            tech.id,
            tech.name,
            tech.father_id,
        )
        .execute(pool)
        .await
        .context("failed to insert tech l2")?;
    }

    Ok(())
}

pub async fn insert_document_tech(
    pool: &sqlx::PgPool,
    doc_techs: Vec<DocumentTech>,
) -> anyhow::Result<()> {
    for doc_tech in doc_techs {
        sqlx::query!(
            r#"
            INSERT INTO documenttech (docid, techid)
            VALUES ($1, $2)
            "#,
            doc_tech.doc_id,
            doc_tech.tech_id,
        )
        .execute(pool)
        .await
        .context("failed to insert document tech")?;
    }

    Ok(())
}

pub async fn insert_system_l1(pool: &sqlx::PgPool, systems: Vec<SystemL1>) -> anyhow::Result<()> {
    for system in systems {
        sqlx::query!(
            r#"
            INSERT INTO systeml1 (id, name)
            VALUES ($1, $2)
            "#,
            system.id,
            system.name,
        )
        .execute(pool)
        .await
        .context("failed to insert system l1")?;
    }

    Ok(())
}

pub async fn insert_system_l2(pool: &sqlx::PgPool, systems: Vec<SystemL2>) -> anyhow::Result<()> {
    for system in systems {
        sqlx::query!(
            r#"
            INSERT INTO systeml2 (id, name, fatherid)
            VALUES ($1, $2, $3)
            "#,
            system.id,
            system.name,
            system.father_id,
        )
        .execute(pool)
        .await
        .context("failed to insert system l2")?;
    }

    Ok(())
}

pub async fn insert_document_system(
    pool: &sqlx::PgPool,
    doc_systems: Vec<DocumentSystem>,
) -> anyhow::Result<()> {
    for doc_system in doc_systems {
        sqlx::query!(
            r#"
            INSERT INTO documentsystem (docid, sysid)
            VALUES ($1, $2)
            "#,
            doc_system.doc_id,
            doc_system.sys_id,
        )
        .execute(pool)
        .await
        .context("failed to insert document system")?;
    }

    Ok(())
}

pub async fn insert_mf_l1(pool: &sqlx::PgPool, mfs: Vec<MfL1>) -> anyhow::Result<()> {
    for mf in mfs {
        sqlx::query!(
            r#"
            INSERT INTO mfl1 (id, name)
            VALUES ($1, $2)
            "#,
            mf.id,
            mf.name,
        )
        .execute(pool)
        .await
        .context("failed to inesrt mf l1")?;
    }

    Ok(())
}

pub async fn insert_mf_l2(pool: &sqlx::PgPool, mfs: Vec<MfL2>) -> anyhow::Result<()> {
    for mf in mfs {
        sqlx::query!(
            r#"
            INSERT INTO mfl2 (id, name, fatherid)
            VALUES ($1, $2, $3)
            "#,
            mf.id,
            mf.name,
            mf.father_id,
        )
        .execute(pool)
        .await
        .context("failed to insert mf l2")?;
    }

    Ok(())
}

pub async fn insert_document_mf(
    pool: &sqlx::PgPool,
    doc_mfs: Vec<DocumentMf>,
) -> anyhow::Result<()> {
    for doc_mf in doc_mfs {
        sqlx::query!(
            r#"
            INSERT INTO documentmf (docid, mfid)
            VALUES ($1, $2)
            "#,
            doc_mf.doc_id,
            doc_mf.mf_id,
        )
        .execute(pool)
        .await
        .context("failed to inesrt document mf")?;
    }

    Ok(())
}

pub async fn insert_project_l1(
    pool: &sqlx::PgPool,
    projects: Vec<ProjectL1>,
) -> anyhow::Result<()> {
    for project in projects {
        sqlx::query!(
            r#"
            INSERT INTO projectl1 (id, name)
            VALUES ($1, $2)
            "#,
            project.id,
            project.name,
        )
        .execute(pool)
        .await
        .context("failed to insert project l1")?;
    }

    Ok(())
}

pub async fn insert_project_l2(
    pool: &sqlx::PgPool,
    projects: Vec<ProjectL2>,
) -> anyhow::Result<()> {
    for project in projects {
        sqlx::query!(
            r#"
            INSERT INTO projectl2 (id, name, fatherid)
            VALUES ($1, $2, $3)
            "#,
            project.id,
            project.name,
            project.father_id,
        )
        .execute(pool)
        .await
        .context("failed to insert project l2")?;
    }

    Ok(())
}

pub async fn insert_document_project(
    pool: &sqlx::PgPool,
    doc_projects: Vec<DocumentProject>,
) -> anyhow::Result<()> {
    let mut doc_projects = doc_projects;
    //let old_size = doc_projects.len();
    doc_projects.sort();
    doc_projects.dedup();

    //assert_eq!(old_size, doc_projects.len());
    for doc_project in doc_projects {
        let result = sqlx::query!(
            r#"
            INSERT INTO documentproject (docid, projectid)
            VALUES ($1, $2)
            "#,
            doc_project.doc_id,
            doc_project.project_id,
        )
        .execute(pool)
        .await;
        //.context("failed to insert document project")?;
        match result {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "failed to insert document project[{}, {}]: {:?}",
                    doc_project.doc_id, doc_project.project_id, e
                );
                return Err(e)?;
            }
        }
    }

    Ok(())
}

pub async fn insert_document_aspice(
    pool: &sqlx::PgPool,
    doc_aspices: Vec<DocumentAspiceMapping>,
) -> anyhow::Result<()> {
    for doc_aspice in doc_aspices {
        sqlx::query!(
            r#"
            INSERT INTO document_aspice_mapping (docid, aspice_step)
            VALUES ($1, $2)
            "#,
            doc_aspice.docid,
            doc_aspice.aspice_step as Aspice,
        )
        .execute(pool)
        .await
        .context("failed to insert document aspice")?;
    }

    Ok(())
}

// 由于数据量比较小，所以在这里直接读取数据库中的数据，然后将其存入HashMap中
// 目的是数据增量导入操作和数据库检索操作解耦。
pub async fn read_exist_documents(
    pool: &sqlx::PgPool,
    docs: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_docs = sqlx::query!(
        r#"
        SELECT id, name
        FROM documents
        "#,
    );

    for doc in db_docs.fetch_all(pool).await? {
        docs.insert(doc.name, doc.id);
    }

    Ok(())
}

pub async fn read_exist_tech_l1(
    pool: &sqlx::PgPool,
    techs: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_techs = sqlx::query!(
        r#"
        SELECT id, name
        FROM techl1
        "#,
    );

    for tech in db_techs.fetch_all(pool).await? {
        techs.insert(tech.name, tech.id);
    }

    Ok(())
}

pub async fn read_exist_tech_l2(
    pool: &sqlx::PgPool,
    techs: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_techs = sqlx::query!(
        r#"
        SELECT id, name
        FROM techl2
        "#,
    );

    for tech in db_techs.fetch_all(pool).await? {
        techs.insert(tech.name, tech.id);
    }

    Ok(())
}

pub async fn read_exist_system_l1(
    pool: &sqlx::PgPool,
    systems: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_systems = sqlx::query!(
        r#"
        SELECT id, name
        FROM systeml1
        "#,
    );

    for system in db_systems.fetch_all(pool).await? {
        systems.insert(system.name, system.id);
    }

    Ok(())
}

pub async fn read_exist_system_l2(
    pool: &sqlx::PgPool,
    systems: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_systems = sqlx::query!(
        r#"
        SELECT id, name
        FROM systeml2
        "#,
    );

    for system in db_systems.fetch_all(pool).await? {
        systems.insert(system.name, system.id);
    }

    Ok(())
}

pub async fn read_exist_mf_l1(
    pool: &sqlx::PgPool,
    mfs: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_mfs = sqlx::query!(
        r#"
        SELECT id, name
        FROM mfl1
        "#,
    );

    for mf in db_mfs.fetch_all(pool).await? {
        mfs.insert(mf.name, mf.id);
    }

    Ok(())
}

pub async fn read_exist_mf_l2(
    pool: &sqlx::PgPool,
    mfs: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_mfs = sqlx::query!(
        r#"
        SELECT id, name
        FROM mfl2
        "#,
    );

    for mf in db_mfs.fetch_all(pool).await? {
        mfs.insert(mf.name, mf.id);
    }

    Ok(())
}

pub async fn read_exist_project_l1(
    pool: &sqlx::PgPool,
    projects: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_projects = sqlx::query!(
        r#"
        SELECT id, name
        FROM projectl1
        "#,
    );

    for project in db_projects.fetch_all(pool).await? {
        projects.insert(project.name, project.id);
    }

    Ok(())
}

pub async fn read_exist_project_l2(
    pool: &sqlx::PgPool,
    projects: &mut HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let db_projects = sqlx::query!(
        r#"
        SELECT id, name
        FROM projectl2
        "#,
    );

    for project in db_projects.fetch_all(pool).await? {
        projects.insert(project.name, project.id);
    }

    Ok(())
}

pub async fn read_exist_document_tech(
    pool: &sqlx::PgPool,
    doc_techs: &mut HashMap<Uuid, HashSet<String>>,
) -> anyhow::Result<()> {
    // join with techl2
    let db_doc_techs = sqlx::query!(
        r#"
        SELECT docid, techl2.name AS techname
        FROM documenttech
        INNER JOIN techl2
        ON documenttech.techid = techl2.id
        "#,
    );

    for doc_tech in db_doc_techs.fetch_all(pool).await? {
        doc_techs
            .entry(doc_tech.docid)
            .or_insert(HashSet::new())
            .insert(doc_tech.techname);
    }

    Ok(())
}

pub async fn read_exist_document_system(
    pool: &sqlx::PgPool,
    doc_systems: &mut HashMap<Uuid, HashSet<String>>,
) -> anyhow::Result<()> {
    // join with systeml2
    let db_doc_systems = sqlx::query!(
        r#"
        SELECT docid, systeml2.name AS systemname
        FROM documentsystem
        INNER JOIN systeml2
        ON documentsystem.sysid = systeml2.id
        "#,
    );

    for doc_system in db_doc_systems.fetch_all(pool).await? {
        doc_systems
            .entry(doc_system.docid)
            .or_insert(HashSet::new())
            .insert(doc_system.systemname);
    }

    Ok(())
}

pub async fn read_exist_document_mf(
    pool: &sqlx::PgPool,
    doc_mfs: &mut HashMap<Uuid, HashSet<String>>,
) -> anyhow::Result<()> {
    // join with mfl2
    let db_doc_mfs = sqlx::query!(
        r#"
        SELECT docid, mfl2.name AS mf_name
        FROM documentmf
        INNER JOIN mfl2
        ON documentmf.mfid = mfl2.id
        "#,
    );

    for doc_mf in db_doc_mfs.fetch_all(pool).await? {
        doc_mfs
            .entry(doc_mf.docid)
            .or_insert(HashSet::new())
            .insert(doc_mf.mf_name);
    }

    Ok(())
}

pub async fn read_exist_document_project(
    pool: &sqlx::PgPool,
    doc_projects: &mut HashMap<Uuid, HashSet<String>>,
) -> anyhow::Result<()> {
    // join with projectl2
    let db_doc_projects = sqlx::query!(
        r#"
        SELECT docid, projectl2.name AS project_name
        FROM documentproject
        INNER JOIN projectl2
        ON documentproject.projectid = projectl2.id
        "#,
    );

    for doc_project in db_doc_projects.fetch_all(pool).await? {
        doc_projects
            .entry(doc_project.docid)
            .or_insert(HashSet::new())
            .insert(doc_project.project_name);
    }

    Ok(())
}

pub async fn read_exist_document_aspice(
    pool: &sqlx::PgPool,
    doc_aspices: &mut HashMap<Uuid, HashSet<Aspice>>,
) -> anyhow::Result<()> {
    let doc_aspice = sqlx::query_as!(
        DocumentAspiceMapping,
        r#"
        SELECT docid, aspice_step as "aspice_step: Aspice"
        FROM document_aspice_mapping
        "#,
    ).fetch_all(pool).await?;

    for doc_aspice in doc_aspice {
        doc_aspices
            .entry(doc_aspice.docid)
            .or_insert(HashSet::new())
            .insert(doc_aspice.aspice_step);
    }

    Ok(())
}
// ----------------------------backend--------------------------------------
