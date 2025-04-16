// use anyhow::anyhow;
use arch_map::database::dml_interface::*;
use arch_map::database::models::*;
use axum::extract::Path;
use axum::response::Json;
use serde_json::Value as JsonValue;
use serde_json::{json, to_value};
// use sqlx::any;
use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
// use std::hash::Hash;
use uuid::Uuid;
/*
{
    'tag': 'filter_tag_name',
    'text': {
        'all': [doc1, doc2],
        'arch': [
            {
                'name': 'techl2_tag1',
                'text': [
                    {
                        'name': doca_name,
                        'address': doca_address,
                        'isRelated': true or false,
                    },
                    doc_b,
                    ...
                 ],
            },
            {
                ...
            },
        ],
        'component': {},
        'mf': {},
        'project': {},
    }
}

*/

pub async fn filter_and_classify(Path(tag_name): Path<String>) -> Json<JsonValue> {
    let mut response: HashMap<String, JsonValue> = HashMap::new();
    response.insert(
        "tag".to_string(),
        to_value(String::from(&tag_name)).unwrap(),
    );
    response.insert(
        "text".to_string(),
        to_value(HashMap::<String, JsonValue>::new()).unwrap(),
    );

    let mut module_type = Some("arch");
    let mut docs = query_docs_by_techl2(&tag_name).await.unwrap();
    if docs.is_empty() {
        docs = query_docs_by_systeml2(&tag_name).await.unwrap();
        module_type = Some("component");
    }
    if docs.is_empty() {
        docs = query_docs_by_mfl2(&tag_name).await.unwrap();
        module_type = Some("mf");
    }
    if docs.is_empty() {
        docs = query_docs_by_projectl2(&tag_name).await.unwrap();
        module_type = Some("project");
    }
    if docs.is_empty() {
        module_type = None;
        return Json(json!("No documents"));
    }

    //let docs = docs.unwrap();

    // Get a mutable reference to the "text" object
    if let Some(text_obj) = response.get_mut("text") {
        if let Some(text_map) = text_obj.as_object_mut() {
            text_map.insert(
                "all".to_string(),
                docs.iter().map(|doc| doc.to_json_value(true)).collect(),
            );
            if module_type.unwrap() != "arch" {
                text_map.insert("arch".to_string(), group_by_techl2(&docs).await.unwrap());
            }

            if module_type.unwrap() != "component" {
                text_map.insert(
                    "component".to_string(),
                    group_by_systeml2(&docs).await.unwrap(),
                );
            }

            if module_type.unwrap() != "mf" {
                text_map.insert("mf".to_string(), group_by_mfl2(&docs).await.unwrap());
            } else {
                text_map.insert(
                    "request".to_string(),
                    get_aspice_graph(&tag_name).await.unwrap(),
                );
            }

            if module_type.unwrap() != "project" {
                let res = group_by_projectl2(&docs).await.unwrap();
                text_map.insert("project".to_string(), res.1);
                text_map.insert("mainLine".to_string(), res.0);
            }
        } else {
            println!("'text' is not an object");
        }
    } else {
        println!("'text' key not found");
    }
    Json(to_value(response).unwrap())
}

/*
[
    {
        'label': 'techl1_tag1',
        'level': 1,
        'children': [
            {
                'label': 'techl2_tag1',
                'level': 2,
                'content': [], # correspond components
                'textNum': $num,
            }
        ]
    },
    ...
]
*/



pub async fn arch_tree(Path(cross_module): Path<String>) -> Json<JsonValue> {
    let db = get_db_pool().await.unwrap();
    let records = sqlx::query!(
        r#"
        SELECT techl1.name as l1name, techl2.name as l2name
        FROM techl1
        INNER JOIN techl2 ON techl1.id = techl2.fatherid
    "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    let mut layered_tags = HashMap::new();
    let mut correspond_tags = HashMap::new();
    for record in records {
        layered_tags
            .entry(record.l1name)
            .or_insert_with(Vec::new)
            .push(record.l2name.clone());

        match cross_module.as_str() {
            "component" => {
                correspond_tags.insert(
                    record.l2name.clone(),
                    arch_correspond_components(&record.l2name).await,
                );
            }
            "mf" => {
                correspond_tags.insert(
                    record.l2name.clone(),
                    arch_correspond_mfs(&record.l2name).await,
                );
            }
            _ => {
                panic!("Invalid cross module");
            }
        }
    }

    let mut result_vec = Vec::new();
    for (l1name, l2names) in layered_tags.into_iter() {
        let mut l2_vec = Vec::new();
        for l2name in l2names {
            let mut l2_obj = json!({
                "label": l2name,
                "level": 2,
                "textNum": query_docs_by_techl2(&l2name).await.unwrap().len(),
            });
            if let Some(correspond_tags) = correspond_tags.get(&l2name) {
                l2_obj["content"] = to_value(correspond_tags.clone()).unwrap();
            }
            l2_vec.push(l2_obj);
        }

        l2_vec.sort_by(|a, b| {
            let a_label = a["label"].as_str().unwrap_or("");
            let b_label = b["label"].as_str().unwrap_or("");
            a_label.cmp(b_label)
        });

        let l1_obj = json!({
            "label": l1name,
            "level": 1,
            "children": l2_vec,
        });
        result_vec.push(l1_obj);
    }

    result_vec.sort_by(|a, b| TECHL1COMP.compare(a["label"].as_str().unwrap(), b["label"].as_str().unwrap()));
    Json(to_value(result_vec).unwrap())
}

pub async fn component_tree(Path(cross_module): Path<String>) -> Json<JsonValue> {
    let db = get_db_pool().await.unwrap();
    let records = sqlx::query!(
        r#"
        SELECT systeml1.name as l1name, systeml2.name as l2name
        FROM systeml1
        INNER JOIN systeml2 ON systeml1.id = systeml2.fatherid
    "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    let mut layered_tags = HashMap::new();
    let mut correspond_tags = HashMap::new();
    for record in records {
        layered_tags
            .entry(record.l1name)
            .or_insert_with(Vec::new)
            .push(record.l2name.clone());

        match cross_module.as_str() {
            "arch" => {
                correspond_tags.insert(
                    record.l2name.clone(),
                    component_cross_archs(&record.l2name).await,
                );
            }
            _ => {
                panic!("Invalid cross module")
            }
        }
    }

    let mut result_vec = Vec::new();
    for (l1name, l2names) in layered_tags.into_iter() {
        let mut l2_vec = Vec::new();
        for l2name in l2names {
            let mut l2_obj = json!({
                "label": l2name,
                "level": 2,
                "textNum": query_docs_by_systeml2(&l2name).await.unwrap().len(),
            });
            if let Some(correspond_tags) = correspond_tags.get(&l2name) {
                l2_obj["content"] = to_value(correspond_tags.clone()).unwrap();
            }
            l2_vec.push(l2_obj);
        }

        l2_vec.sort_by(|a, b| {
            let a_label = a["label"].as_str().unwrap_or("");
            let b_label = b["label"].as_str().unwrap_or("");
            a_label.cmp(b_label)
        });

        let l1_obj = json!({
            "label": l1name,
            "level": 1,
            "children": l2_vec,
        });
        result_vec.push(l1_obj);
    }
    result_vec.sort_by(|a, b| {
        let a_label = a["label"].as_str().unwrap_or("");
        let b_label = b["label"].as_str().unwrap_or("");
        a_label.cmp(b_label)
    });

    Json(to_value(result_vec).unwrap())
}

pub async fn mf_tree(Path(cross_module): Path<String>) -> Json<JsonValue> {
    let db = get_db_pool().await.unwrap();
    let records = sqlx::query!(
        r#"
        SELECT mfl1.name as l1name, mfl2.name as l2name
        FROM mfl1
        INNER JOIN mfl2 ON mfl1.id = mfl2.fatherid
    "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    let mut layered_tags = HashMap::new();
    let mut correspond_tags = HashMap::new();
    for record in records {
        layered_tags
            .entry(record.l1name)
            .or_insert_with(Vec::new)
            .push(record.l2name.clone());

        match cross_module.as_str() {
            "arch" => {
                correspond_tags.insert(record.l2name.clone(), mf_cross_archs(&record.l2name).await);
            }
            _ => {
                panic!("Invalid cross module")
            }
        }
    }

    let mut result_vec = Vec::new();
    for (l1name, l2names) in layered_tags.into_iter() {
        let mut l2_vec = Vec::new();
        for l2name in l2names {
            let mut l2_obj = json!({
                "label": l2name,
                "level": 2,
                "textNum": query_docs_by_mfl2(&l2name).await.unwrap().len(),
            });
            if let Some(correspond_tags) = correspond_tags.get(&l2name) {
                l2_obj["content"] = to_value(correspond_tags.clone()).unwrap();
            }
            l2_vec.push(l2_obj);
        }

        l2_vec.sort_by(|a, b| {
            let a_label = a["label"].as_str().unwrap_or("");
            let b_label = b["label"].as_str().unwrap_or("");
            a_label.cmp(b_label)
        });

        let l1_obj = json!({
            "label": l1name,
            "level": 1,
            "children": l2_vec,
        });
        result_vec.push(l1_obj);
    }

    result_vec.sort_by(|a, b| {
        let a_label = a["label"].as_str().unwrap_or("");
        let b_label = b["label"].as_str().unwrap_or("");
        a_label.cmp(b_label)
    });

    Json(to_value(result_vec).unwrap())
}

/*
       text:[
               {
                   name: '传感器方案',
                   level: 1,
                   children: [
                       {
                           name: '传感器布局',
                           level: 2,
                           children:[
                               {
                                   name: 'Rocky AB面传感器配置',
                                   address: 'https://momenta.feishu.cn/wiki/ORuBw5pT8iAUmPkkaEYcRsc1nLh'
                               },
                               {
                                   name: 'Rocky sensor set with ASIL_v0.0.4_0330.xlsx',
                                   address: 'https://momenta.feishu.cn/wiki/HC1uwEdemiusn3k7shGcwUZMnYe'
                               }
                           ]
                       },
                   ]
               },
       ]
*/
pub async fn project_arch_tree(Path(project_name): Path<String>) -> Json<JsonValue> {
    let db = get_db_pool().await.unwrap();
    let docs_having_project_tag = sqlx::query!(
        r#"
        SELECT documentproject.docid
        FROM documentproject
        INNER JOIN projectl2 ON documentproject.projectid = projectl2.id
        WHERE projectl2.name = $1
    "#,
        project_name
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.docid)
    .collect::<Vec<Uuid>>();

    let records = sqlx::query!(r#"
        SELECT techl1.name as l1name, techl2.name as l2name, documenttech.docid, documents.name as docname, documents.link as doclink
        FROM techl1
        INNER JOIN techl2 ON techl1.id = techl2.fatherid
        INNER JOIN documenttech ON techl2.id = documenttech.techid
        INNER JOIN documents ON documenttech.docid = documents.id
    "#).fetch_all(&db).await.unwrap().into_iter().filter(|record| docs_having_project_tag.contains(&record.docid));

    let mut layered_tags = HashMap::new();
    let mut correspond_docs = HashMap::new();
    for record in records {
        layered_tags
            .entry(record.l1name)
            .or_insert_with(HashSet::new)
            .insert(record.l2name.clone());

        correspond_docs
            .entry(record.l2name)
            .or_insert_with(Vec::new)
            .push(json!({
                "name": record.docname,
                "address": record.doclink,
            }));
    }

    let mut arch_vec = layered_tags
        .into_iter()
        .map(|(l1name, l2names)| {
            let mut l2_vec = l2names
                .into_iter()
                .map(|l2name| {
                    let mut l2_obj = json!({
                        "name": l2name,
                        "level": 2,
                    });
                    if let Some(correspond_docs) = correspond_docs.get(&l2name) {
                        l2_obj["children"] = to_value(correspond_docs.clone()).unwrap();
                    }
                    l2_obj
                })
                .collect::<Vec<JsonValue>>();

            l2_vec.sort_by(|a, b| {
                let a_label = a["name"].as_str().unwrap_or("");
                let b_label = b["name"].as_str().unwrap_or("");
                a_label.cmp(b_label)
            });

            let l1_obj = json!({
                "name": l1name,
                "level": 1,
                "children": l2_vec,
            });
            l1_obj
        })
        .collect::<Vec<JsonValue>>();

    arch_vec.sort_by(|a, b| TECHL1COMP.compare(a["name"].as_str().unwrap(), b["name"].as_str().unwrap()));

    Json(to_value(arch_vec).unwrap())
}

async fn arch_correspond_components(techl2: &str) -> Vec<String> {
    let db = get_db_pool().await.unwrap();
    // techl2_name | techl2_id | doc_id
    let docs_having_techl2 = sqlx::query!(
        r#"
        SELECT documenttech.docid
        FROM techl2
        INNER JOIN Documenttech ON techl2.id = documenttech.techid
        WHERE techl2.name = $1
    "#,
        techl2
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.docid)
    .collect::<HashSet<Uuid>>();

    let result = sqlx::query!(
        r#"
        SELECT systeml2.name, documentsystem.docid
        FROM systeml2
        INNER JOIN documentsystem ON systeml2.id = documentsystem.sysid
        "#
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .filter(|record| docs_having_techl2.contains(&record.docid))
    .map(|record| record.name)
    .collect::<HashSet<String>>();

    result.into_iter().collect()
}

async fn arch_correspond_mfs(techl2: &str) -> Vec<String> {
    let db = get_db_pool().await.unwrap();
    // techl2_name | techl2_id | doc_id
    let docs_having_techl2 = sqlx::query!(
        r#"
        SELECT documenttech.docid
        FROM techl2
        INNER JOIN Documenttech ON techl2.id = documenttech.techid
        WHERE techl2.name = $1
    "#,
        techl2
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.docid)
    .collect::<Vec<Uuid>>();

    let mut result = sqlx::query!(
        r#"
        SELECT mfl2.name, documentmf.docid
        FROM mfl2
        INNER JOIN documentmf ON mfl2.id = documentmf.mfid
        "#
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .filter(|record| docs_having_techl2.contains(&record.docid))
    .map(|record| record.name)
    .collect::<Vec<String>>();

    result.sort();
    result.dedup();
    result
}

async fn component_cross_archs(systeml2: &str) -> Vec<String> {
    let db = get_db_pool().await.unwrap();
    // systeml2_name | systeml2_id | doc_id
    let docs_having_systeml2 = sqlx::query!(
        r#"
        SELECT documentsystem.docid
        FROM systeml2
        INNER JOIN documentsystem ON systeml2.id = documentsystem.sysid
        WHERE systeml2.name = $1
    "#,
        systeml2
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.docid)
    .collect::<Vec<Uuid>>();

    let mut result = sqlx::query!(
        r#"
        SELECT techl2.name, documenttech.docid
        FROM techl2
        INNER JOIN documenttech ON techl2.id = documenttech.techid
        "#
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .filter(|record| docs_having_systeml2.contains(&record.docid))
    .map(|record| record.name)
    .collect::<Vec<String>>();

    result.sort();
    result.dedup();
    result
}

async fn mf_cross_archs(mfl2: &str) -> Vec<String> {
    let db = get_db_pool().await.unwrap();
    // mfl2_name | mfl2_id | doc_id
    let docs_having_mfl2 = sqlx::query!(
        r#"
        SELECT documentmf.docid
        FROM mfl2
        INNER JOIN documentmf ON mfl2.id = documentmf.mfid
        WHERE mfl2.name = $1
    "#,
        mfl2
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.docid)
    .collect::<Vec<Uuid>>();

    let mut result = sqlx::query!(
        r#"
        SELECT techl2.name, documenttech.docid
        FROM techl2
        INNER JOIN documenttech ON techl2.id = documenttech.techid
        "#
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .filter(|record| docs_having_mfl2.contains(&record.docid))
    .map(|record| record.name)
    .collect::<Vec<String>>();

    result.sort();
    result.dedup();
    result
}

/*
 */

async fn query_docs_by_techl2(tag_name: &str) -> anyhow::Result<Vec<Document>> {
    let db = get_db_pool().await?;

    let mut docs = sqlx::query!(
        r#"
        SELECT documents.* FROM documents
        INNER JOIN documenttech ON documents.id = documenttech.docid
        INNER JOIN techl2 ON documenttech.techid = techl2.id
        WHERE techl2.name = $1
        ORDER BY documents.id
    "#,
        tag_name
    )
    .fetch_all(&db)
    .await?;

    let docs: Vec<Document> = docs
        .iter_mut()
        .map(|doc| Document {
            id: doc.id,
            name: doc.name.clone(),
            link: doc.link.clone(),
            description: doc.description.clone(),
            associate_requirement: doc.associate_requirement,
        })
        .collect();

    Ok(docs)
}

async fn query_docs_by_systeml2(tag_name: &str) -> anyhow::Result<Vec<Document>> {
    let db = get_db_pool().await?;
    let docs = sqlx::query_as!(
        Document,
        r#"
        SELECT documents.* FROM documents
        INNER JOIN documentsystem ON documents.id = documentsystem.docid
        INNER JOIN systeml2 ON documentsystem.sysid = systeml2.id
        WHERE systeml2.name = $1
        ORDER BY documents.id
    "#,
        tag_name
    )
    .fetch_all(&db)
    .await?;

    Ok(docs)
}

async fn query_docs_by_mfl2(tag_name: &str) -> anyhow::Result<Vec<Document>> {
    let db = get_db_pool().await?;
    Ok(sqlx::query_as!(
        Document,
        r#"
        SELECT documents.* FROM documents
        INNER JOIN documentmf ON documents.id = documentmf.docid
        INNER JOIN mfl2 ON documentmf.mfid = mfl2.id
        WHERE mfl2.name = $1
        ORDER BY documents.id
    "#,
        tag_name
    )
    .fetch_all(&db)
    .await?)
}

async fn query_docs_by_projectl2(tag_name: &str) -> anyhow::Result<Vec<Document>> {
    let db = get_db_pool().await?;
    Ok(sqlx::query_as!(
        Document,
        r#"
        SELECT documents.* FROM documents
        INNER JOIN documentproject ON documents.id = documentproject.docid
        INNER JOIN projectl2 ON documentproject.projectid = projectl2.id
        WHERE projectl2.name = $1
        ORDER BY documents.id
    "#,
        tag_name
    )
    .fetch_all(&db)
    .await?)
}

// struct TagFilterResponse {
//     tag: String,
//     text: HashMap<String, JsonValue>,
// }

// fn check_documents_order(docs: &Vec<Document>) -> bool {
//     docs.windows(2).all(|w| w[0] <= w[1])
// }

/*
[
    {
        'name': 'techl2_tag1',
        'text': [
            {
                'name': doca_name,
                'address': doca_address,
                'isRelated': true or false,//该文档是否和当前系统部件关联
            },
            doc_b
        ],
    },
    {
        ...
    },
],
*/
async fn group_by_techl2(docs: &Vec<Document>) -> anyhow::Result<JsonValue> {
    //assert!(check_documents_order(docs));
    let db = get_db_pool().await?;
    let records = sqlx::query!(
        r#"
        SELECT techl2.name, documenttech.docid FROM techl2
        INNER JOIN documenttech ON techl2.id = documenttech.techid
        GROUP BY techl2.name, documenttech.docid
    "#
    )
    .fetch_all(&db)
    .await?;

    let mut grouped: HashMap<String, VecDeque<JsonValue>> = HashMap::new();

    for record in records {
        let techl2_name = record.name;
        let docid = record.docid;
        let doc = docs.iter().find(|doc| doc.id == docid);
        if let Some(doc) = doc {
            grouped
                .entry(techl2_name)
                .or_insert_with(VecDeque::new)
                .push_front(doc.to_json_value(true));
        } else {
            let doc = sqlx::query_as!(
                Document,
                r#"
                SELECT * FROM documents WHERE id = $1
            "#,
                docid
            )
            .fetch_one(&db)
            .await?;
            grouped
                .entry(techl2_name)
                .and_modify(|f| f.push_back(doc.to_json_value(false)));
        }
    }

    let mut result_vec = Vec::new();

    for (name, text) in grouped.into_iter() {
        let techl2_obj = json!({
            "name": name,
            "text": text,
        });

        result_vec.push(techl2_obj);
    }

    Ok(JsonValue::Array(result_vec))
}

async fn group_by_systeml2(docs: &Vec<Document>) -> anyhow::Result<JsonValue> {
    let db = get_db_pool().await?;
    let records = sqlx::query!(
        r#"
        SELECT systeml2.name, documentsystem.docid FROM systeml2
        INNER JOIN documentsystem ON systeml2.id = documentsystem.sysid
        GROUP BY systeml2.name, documentsystem.docid
    "#
    )
    .fetch_all(&db)
    .await?;

    let mut grouped = HashMap::new();

    for record in records {
        let systeml2_name = record.name;
        let docid = record.docid;
        let doc = docs.iter().find(|doc| doc.id == docid);
        if let Some(doc) = doc {
            grouped
                .entry(systeml2_name)
                .or_insert_with(VecDeque::new)
                .push_front(doc.to_json_value(true));
        } else {
            let doc = sqlx::query_as!(
                Document,
                r#"
                SELECT * FROM documents WHERE id = $1
            "#,
                docid
            )
            .fetch_one(&db)
            .await?;
            grouped
                .entry(systeml2_name)
                .and_modify(|f| f.push_back(doc.to_json_value(false)));
        }
    }

    let mut result_vec = Vec::new();
    for (name, text) in grouped.into_iter() {
        let systeml2_obj = json!({
            "name": name,
            "text": text,
        });

        result_vec.push(systeml2_obj);
    }

    Ok(JsonValue::Array(result_vec))
}

async fn group_by_mfl2(docs: &Vec<Document>) -> anyhow::Result<JsonValue> {
    let db = get_db_pool().await?;
    let records = sqlx::query!(
        r#"
        SELECT mfl2.name, documentmf.docid FROM mfl2
        INNER JOIN documentmf ON mfl2.id = documentmf.mfid
        GROUP BY mfl2.name, documentmf.docid
    "#
    )
    .fetch_all(&db)
    .await?;

    let mut grouped = HashMap::new();

    for record in records {
        let mfl2_name = record.name;
        let docid = record.docid;
        let doc = docs.iter().find(|doc| doc.id == docid);
        if let Some(doc) = doc {
            grouped
                .entry(mfl2_name)
                .or_insert_with(VecDeque::new)
                .push_front(doc.to_json_value(true));
        } else {
            let doc = sqlx::query_as!(
                Document,
                r#"
                SELECT * FROM documents WHERE id = $1
            "#,
                docid
            )
            .fetch_one(&db)
            .await?;
            grouped
                .entry(mfl2_name)
                .and_modify(|f| f.push_back(doc.to_json_value(false)));
        }
    }

    let mut result_vec = Vec::new();
    for (name, text) in grouped.into_iter() {
        let mfl2_obj = json!({
            "name": name,
            "text": text,
        });

        result_vec.push(mfl2_obj);
    }

    Ok(JsonValue::Array(result_vec))
}

// return (mainline, projects)
async fn group_by_projectl2(docs: &Vec<Document>) -> anyhow::Result<(JsonValue, JsonValue)> {
    let db = get_db_pool().await?;
    let records = sqlx::query!(
        r#"
        SELECT projectl2.name, documentproject.docid FROM projectl2
        INNER JOIN documentproject ON projectl2.id = documentproject.projectid
        GROUP BY projectl2.name, documentproject.docid
    "#
    )
    .fetch_all(&db)
    .await?;

    let mut grouped = HashMap::new();

    for record in records {
        let projectl2_name = record.name;
        let docid = record.docid;
        let doc = docs.iter().find(|doc| doc.id == docid);
        if let Some(doc) = doc {
            grouped
                .entry(projectl2_name)
                .or_insert_with(VecDeque::new)
                .push_front(doc.to_json_value(true));
        } else {
            let doc = sqlx::query_as!(
                Document,
                r#"
                SELECT * FROM documents WHERE id = $1
            "#,
                docid
            )
            .fetch_one(&db)
            .await?;
            grouped
                .entry(projectl2_name)
                .and_modify(|f| f.push_back(doc.to_json_value(false)));
        }
    }

    let mut project_vec = Vec::new();
    let mut mainline_vec = Vec::new();
    for (name, text) in grouped.into_iter() {
        let projectl2_obj = json!({
            "name": name,
            "text": text,
        });

        if name == "主线" {
            mainline_vec.push(projectl2_obj);
        } else {
            project_vec.push(projectl2_obj);
        }
    }

    Ok((
        JsonValue::Array(mainline_vec),
        JsonValue::Array(project_vec),
    ))
}

// request:[
//     {
//         name: '需求1',
//         content:[
//             {
//                 name: '软件需求分析',
//                 content:[
//                     {
//                         name:'文档名称1',
//                         address: 'https://momenta.feishu.cn/wiki/ORuBw5pT8iAUmPkkaEYcRsc1nLh'
//                     }
//                 ]
//             },
//             {
//                 name: '软件架构设计',
//                 content:[
//                     {
//                         name:'文档名称2',
//                         address: 'https://momenta.feishu.cn/wiki/HC1uwEdemiusn3k7shGcwUZMnYe'
//                     }
//                 ]
//             }
//         ]
//     },
//     {
//         name: '需求2',
//     }
// ],
async fn get_aspice_graph(mf_name: &str) -> anyhow::Result<JsonValue> {
    let db = get_db_pool().await?;

    // find all docs having mf_name; then extract all aspice tags
    let records = sqlx::query!(
        r#"
        SELECT documents.associate_requirement, document_aspice_mapping.aspice_step as "aspice_step: Aspice", documents.name, documents.link
        FROM documents
        INNER JOIN documentmf ON documents.id = documentmf.docid
        INNER JOIN mfl2 ON documentmf.mfid = mfl2.id
        INNER JOIN document_aspice_mapping ON documents.id = document_aspice_mapping.docid
        WHERE mfl2.name = $1
    "#,
        mf_name
    )
    .fetch_all(&db)
    .await?;

    let mut map = HashMap::new();
    for record in records {
        map.entry(record.associate_requirement)
            .or_insert(HashMap::new())
            .entry(record.aspice_step)
            .or_insert(Vec::new())
            .push(json!({
                "name": record.name,
                "address": record.link,
            }));
    }

    let mut result_vec = Vec::new();
    for (requirement, aspice_map) in map.into_iter() {
        let req_name = sqlx::query!(
            r#"
            SELECT name FROM documents WHERE id = $1"#,
            requirement
        )
        .fetch_one(&db)
        .await?
        .name;

        let mut aspice_vec = Vec::new();
        for (aspice_step, docs) in aspice_map.into_iter() {
            let aspice_obj = json!({
                "name": aspice_step.to_string(),
                "content": docs,
            });
            aspice_vec.push(aspice_obj);
        }
        let requirement_obj = json!({
            "name": req_name,
            "content": aspice_vec,
        });
        result_vec.push(requirement_obj);
    }
    Ok(JsonValue::Array(result_vec))
}

lazy_static::lazy_static! {
    static ref TECHL1COMP: TechL1Compare = TechL1Compare::new();
}

struct TechL1Compare {
    predefined_order: HashMap<&'static str, usize>,
}

impl TechL1Compare {
    fn new() -> Self {
        let mut predefined_order = HashMap::new();
        predefined_order.insert("传感器方案", 1);
        predefined_order.insert("域控硬件方案", 2);
        predefined_order.insert("OS+中间件方案", 3);
        predefined_order.insert("执行器交互方案", 4);
        predefined_order.insert("EE架构方案", 5);
        predefined_order.insert("信号接入方案", 6);
        predefined_order.insert("通讯方案", 7);
        predefined_order.insert("应用层基础服务方案", 8);
        predefined_order.insert("系统方案", 9);
        predefined_order.insert("软件优化方案", 10);
        TechL1Compare {
            predefined_order,
        }
    }

    fn compare(&self, s1: &str, s2: &str) -> std::cmp::Ordering {
        let order1 = self.predefined_order.get(s1).unwrap_or(&usize::MAX);
        let order2 = self.predefined_order.get(s2).unwrap_or(&usize::MAX);
        order1.cmp(order2)
    }
}