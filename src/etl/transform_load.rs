use crate::database::dml_interface;
use crate::database::models::*;
/// Transform data from excel to rust internal data structure.
/// And, load data to database.
use anyhow::Context;
use calamine::{Reader, Xlsx};
use sqlx::any;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek};
use std::path::Path;
use uuid::Uuid;

pub struct InternalData {
    pub documents: Vec<Document>,
    pub tech_l1: Vec<TechL1>,
    pub tech_l2: Vec<TechL2>,
    pub document_tech: Vec<DocumentTech>,
    pub system_l1: Vec<SystemL1>,
    pub system_l2: Vec<SystemL2>,
    pub document_system: Vec<DocumentSystem>,
    pub mf_l1: Vec<MfL1>,
    pub mf_l2: Vec<MfL2>,
    pub document_mf: Vec<DocumentMf>,
    pub project_l1: Vec<ProjectL1>,
    pub project_l2: Vec<ProjectL2>,
    pub document_project: Vec<DocumentProject>,
    pub document_aspice: Vec<DocumentAspiceMapping>,

    pub exist_documents: HashMap<String, Uuid>,
    pub exist_tech_l1: HashMap<String, Uuid>,
    pub exist_tech_l2: HashMap<String, Uuid>,
    pub exist_system_l1: HashMap<String, Uuid>,
    pub exist_system_l2: HashMap<String, Uuid>,
    pub exist_mf_l1: HashMap<String, Uuid>,
    pub exist_mf_l2: HashMap<String, Uuid>,
    pub exist_project_l1: HashMap<String, Uuid>,
    pub exist_project_l2: HashMap<String, Uuid>,
    pub exist_document_tech: HashMap<Uuid, HashSet<String>>,
    pub exist_document_system: HashMap<Uuid, HashSet<String>>,
    pub exist_document_mf: HashMap<Uuid, HashSet<String>>,
    pub exist_document_project: HashMap<Uuid, HashSet<String>>,
    pub exist_document_aspice: HashMap<Uuid, HashSet<Aspice>>,
}

impl InternalData {
    pub fn new() -> InternalData {
        InternalData {
            documents: Vec::new(),
            tech_l1: Vec::new(),
            tech_l2: Vec::new(),
            document_tech: Vec::new(),
            system_l1: Vec::new(),
            system_l2: Vec::new(),
            document_system: Vec::new(),
            mf_l1: Vec::new(),
            mf_l2: Vec::new(),
            document_mf: Vec::new(),
            project_l1: Vec::new(),
            project_l2: Vec::new(),
            document_project: Vec::new(),
            document_aspice: Vec::new(),

            exist_documents: HashMap::new(),
            exist_tech_l1: HashMap::new(),
            exist_tech_l2: HashMap::new(),
            exist_system_l1: HashMap::new(),
            exist_system_l2: HashMap::new(),
            exist_mf_l1: HashMap::new(),
            exist_mf_l2: HashMap::new(),
            exist_project_l1: HashMap::new(),
            exist_project_l2: HashMap::new(),

            exist_document_tech: HashMap::new(),
            exist_document_system: HashMap::new(),
            exist_document_mf: HashMap::new(),
            exist_document_project: HashMap::new(),
            exist_document_aspice: HashMap::new(),
        }
    }

    pub async fn import_and_load(&mut self, path: &Path) -> anyhow::Result<()> {
        let pool = dml_interface::get_db_pool().await?;

        self.read_exist_data(&pool).await?;
        self.import_from_excel(path)?;
        self.load_to_database(&pool).await?;

        pool.close().await;
        Ok(())
    }

    pub async fn read_exist_data(&mut self, pool: &sqlx::PgPool) -> anyhow::Result<()> {
        dml_interface::read_exist_documents(&pool, &mut self.exist_documents).await?;
        dml_interface::read_exist_tech_l1(&pool, &mut self.exist_tech_l1).await?;
        dml_interface::read_exist_tech_l2(&pool, &mut self.exist_tech_l2).await?;
        dml_interface::read_exist_system_l1(&pool, &mut self.exist_system_l1).await?;
        dml_interface::read_exist_system_l2(&pool, &mut self.exist_system_l2).await?;
        dml_interface::read_exist_mf_l1(&pool, &mut self.exist_mf_l1).await?;
        dml_interface::read_exist_mf_l2(&pool, &mut self.exist_mf_l2).await?;
        dml_interface::read_exist_project_l1(&pool, &mut self.exist_project_l1).await?;
        dml_interface::read_exist_project_l2(&pool, &mut self.exist_project_l2).await?;

        dml_interface::read_exist_document_tech(&pool, &mut self.exist_document_tech).await?;
        dml_interface::read_exist_document_system(&pool, &mut self.exist_document_system).await?;
        dml_interface::read_exist_document_mf(&pool, &mut self.exist_document_mf).await?;
        dml_interface::read_exist_document_project(&pool, &mut self.exist_document_project).await?;
        dml_interface::read_exist_document_aspice(&pool, &mut self.exist_document_aspice).await?;
        Ok(())
    }

    pub fn import_from_excel(&mut self, path: &Path) -> anyhow::Result<()> {
        let mut excel: Xlsx<_> = calamine::open_workbook(path)?;
        self.import_tech_tags(&mut excel)?;
        self.import_system_tags(&mut excel)?;
        self.import_mf_tags(&mut excel)?;
        self.import_project_tags(&mut excel)?;

        // must after import all tags
        self.import_documents(&mut excel)?;
        Ok(())
    }

    pub async fn load_to_database(&self, pool: &sqlx::PgPool) -> anyhow::Result<()> {
        // TODO: use transaction

        // insert tags
        dml_interface::insert_tech_l1(&pool, self.tech_l1.clone()).await?;
        dml_interface::insert_tech_l2(&pool, self.tech_l2.clone()).await?;
        dml_interface::insert_system_l1(&pool, self.system_l1.clone()).await?;
        dml_interface::insert_system_l2(&pool, self.system_l2.clone()).await?;
        dml_interface::insert_mf_l1(&pool, self.mf_l1.clone()).await?;
        dml_interface::insert_mf_l2(&pool, self.mf_l2.clone()).await?;
        dml_interface::insert_project_l1(&pool, self.project_l1.clone()).await?;
        dml_interface::insert_project_l2(&pool, self.project_l2.clone()).await?;
        dml_interface::insert_documents(&pool, self.documents.clone()).await?;
        dml_interface::insert_document_tech(&pool, self.document_tech.clone()).await?;
        dml_interface::insert_document_system(&pool, self.document_system.clone()).await?;
        dml_interface::insert_document_mf(&pool, self.document_mf.clone()).await?;
        dml_interface::insert_document_project(&pool, self.document_project.clone()).await?;
        dml_interface::insert_document_aspice(&pool, self.document_aspice.clone()).await?;
        Ok(())
    }

    fn import_tech_tags(&mut self, excel: &mut Xlsx<impl Read + Seek>) -> anyhow::Result<()> {
        let sheet_name = "技术方案选项";

        if let Some(Ok(range)) = excel.worksheet_range(sheet_name) {
            let mut current_tl1 = None;
            for row in range.rows().skip(1) {
                // TODO: remove magic number
                let l1_cell = row.get(1).and_then(|cell| cell.get_string());
                let l2_cell = row.get(2).and_then(|cell| cell.get_string());

                if let Some(tl1) = l1_cell {
                    current_tl1 = Some(tl1);
                    if self.exist_tech_l1.contains_key(tl1) == false {
                        let tag = TechL1::new(tl1.to_string());
                        self.exist_tech_l1.insert(tag.name.clone(), tag.id);
                        self.tech_l1.push(tag);
                    }
                }

                if let Some(tl2) = l2_cell {
                    if self.exist_tech_l2.contains_key(tl2) {
                        continue;
                    }

                    let tag = TechL2::new(
                        tl2.to_string(),
                        self.exist_tech_l1
                            .get(current_tl1.unwrap())
                            .unwrap()
                            .to_owned(),
                    );
                    self.exist_tech_l2.insert(tag.name.clone(), tag.id);
                    self.tech_l2.push(tag);
                }
            }
        }

        Ok(())
    }

    fn import_system_tags(&mut self, excel: &mut Xlsx<impl Read + Seek>) -> anyhow::Result<()> {
        let sheet_name = "系统部件选项";

        let mut current_sl1 = None;
        if let Some(Ok(range)) = excel.worksheet_range(sheet_name) {
            for row in range.rows().skip(1) {
                let l1_cell = row.get(1).and_then(|cell| cell.get_string());
                let l2_cell = row.get(2).and_then(|cell| cell.get_string());

                if let Some(sl1) = l1_cell {
                    current_sl1 = Some(sl1);
                    if self.exist_system_l1.contains_key(sl1) == false {
                        let tag = SystemL1::new(sl1.to_string());
                        self.exist_system_l1.insert(tag.name.clone(), tag.id);
                        self.system_l1.push(tag);
                    }
                }

                if let Some(sl2) = l2_cell {
                    if self.exist_system_l2.contains_key(sl2) == false {
                        let tag = SystemL2::new(
                            sl2.to_string(),
                            self.exist_system_l1
                                .get(current_sl1.unwrap())
                                .unwrap()
                                .to_owned(),
                        );
                        self.exist_system_l2.insert(tag.name.clone(), tag.id);
                        self.system_l2.push(tag);
                    }
                }
            }
        }

        Ok(())
    }

    fn import_mf_tags(&mut self, excel: &mut Xlsx<impl Read + Seek>) -> anyhow::Result<()> {
        let sheet_name = "MF软件选项";

        if let Some(Ok(range)) = excel.worksheet_range(sheet_name) {
            let mut current_mfl1 = None;
            for row in range.rows().skip(1) {
                let l1_cell = row.get(1).and_then(|cell| cell.get_string());
                let l2_cell = row.get(2).and_then(|cell| cell.get_string());

                if let Some(mfl1) = l1_cell {
                    current_mfl1 = Some(mfl1);
                    if self.exist_mf_l1.contains_key(mfl1) == false {
                        let tag = MfL1::new(mfl1.to_string());
                        self.exist_mf_l1.insert(tag.name.clone(), tag.id);
                        self.mf_l1.push(tag);
                    }
                }

                if let Some(mfl2) = l2_cell {
                    if self.exist_mf_l2.contains_key(mfl2) == false {
                        let tag = MfL2::new(
                            mfl2.to_string(),
                            self.exist_mf_l1
                                .get(current_mfl1.unwrap())
                                .unwrap()
                                .to_owned(),
                        );
                        self.exist_mf_l2.insert(tag.name.clone(), tag.id);
                        self.mf_l2.push(tag);
                    }
                }
            }
        }

        Ok(())
    }

    fn import_project_tags(&mut self, excel: &mut Xlsx<impl Read + Seek>) -> anyhow::Result<()> {
        let sheet_name = "主线或项目选项";

        if let Some(Ok(range)) = excel.worksheet_range(sheet_name) {
            let mut current_pl1 = None;
            for row in range.rows().skip(1) {
                let l1_cell = row.get(1).and_then(|cell| cell.get_string());
                let l2_cell = row.get(2).and_then(|cell| cell.get_string());

                if let Some(pl1) = l1_cell {
                    current_pl1 = Some(pl1);
                    if self.exist_project_l1.contains_key(pl1) == false {
                        let tag = ProjectL1::new(pl1.to_string());
                        self.exist_project_l1.insert(tag.name.clone(), tag.id);
                        self.project_l1.push(tag);
                    }
                }

                if let Some(pl2) = l2_cell {
                    if self.exist_project_l2.contains_key(pl2) == false {
                        let tag = ProjectL2::new(
                            pl2.to_string(),
                            self.exist_project_l1
                                .get(current_pl1.unwrap())
                                .unwrap()
                                .to_owned(),
                        );
                        self.exist_project_l2.insert(tag.name.clone(), tag.id);
                        self.project_l2.push(tag);
                    }
                }
            }
        }

        Ok(())
    }

    fn import_documents(&mut self, excel: &mut Xlsx<impl Read + Seek>) -> anyhow::Result<()> {
        let sheet_name = "文档管理";

        if let Some(Ok(range)) = excel.worksheet_range(sheet_name) {
            for row in range.rows().skip(1) {
                let name = row.get(0).and_then(|cell| cell.get_string());
                if let Some("") = name {
                    continue;
                }

                // get hyperlink
                let link = row.get(11).and_then(|cell| cell.get_string());

                // TODO: use python ffi to parse hyper link from excel
                let description = row.get(1).and_then(|cell| cell.get_string());
                //let techl1_vec = row.get(2).and_then(|cell| Some(cell.get_string().unwrap().split(",").collect::<Vec<&str>>()));
                let techl2_vec = row.get(3).and_then(|cell| {
                    cell.get_string().map(|str_val| {
                        str_val
                            .trim()
                            .split(",")
                            .map(|s| s.trim())
                            .collect::<Vec<&str>>()
                    })
                });
                //let systeml1_vec = row.get(4).and_then(|cell| cell.get_string()).unwrap().split(",").collect::<Vec<&str>>();
                let systeml2_vec = row.get(5).and_then(|cell| {
                    cell.get_string().map(|str_val| {
                        str_val
                            .trim()
                            .split(",")
                            .map(|s| s.trim())
                            .collect::<Vec<&str>>()
                    })
                });
                //let mfl1_vec = row.get(6).and_then(|cell| cell.get_string()).unwrap().split(",").collect::<Vec<&str>>();
                let mfl2_vec = row.get(7).and_then(|cell| {
                    cell.get_string().map(|str_val| {
                        str_val
                            .trim()
                            .split(",")
                            .map(|s| s.trim())
                            .collect::<Vec<&str>>()
                    })
                });

                //let sf_demond = row.get(8).and_then(|cell| cell.get_string());
                //let aspice_vec = row.get(9).and_then(|cell| cell.get_string()).unwrap().split(",").collect::<Vec<&str>>();
                let projectl2_vec = row.get(10).and_then(|cell| {
                    cell.get_string().map(|str_val| {
                        str_val
                            .trim()
                            .split(",")
                            .map(|s| s.trim())
                            .collect::<Vec<&str>>()
                    })
                });

                let aspice_vec = row.get(9).and_then(|cell| {
                    cell.get_string().map(|str_val| {
                        str_val
                            .trim()
                            .split(",")
                            .map(|s| s.trim())
                            .collect::<Vec<&str>>()
                    })
                });

                let mut current_doc_id = None;
                if let Some(name) = name {
                    // FIXME: fix link
                    if self.exist_documents.contains_key(name) == false {
                        let doc = Document::new(
                            name.to_string(),
                            link.unwrap().to_string(),
                            description.map(|s| s.to_string()),
                        );
                        current_doc_id = Some(doc.id);
                        self.exist_documents.insert(doc.name.clone(), doc.id);
                        self.documents.push(doc);
                    } else {
                        current_doc_id = Some(self.exist_documents.get(name).unwrap().to_owned());
                    }
                }

                if let Some(techl2_vec) = techl2_vec {
                    let exist_tag_set = self.exist_document_tech.entry(current_doc_id.unwrap()).or_insert(HashSet::new());
                    for techl2 in techl2_vec {
                        let techl2_id = self.exist_tech_l2.get(techl2).unwrap().to_owned();
                        if exist_tag_set.contains(techl2) == false {
                            self.document_tech.push(DocumentTech::new(current_doc_id.unwrap(), techl2_id));
                            exist_tag_set.insert(techl2.to_string());
                        }
                    }
                }

                if let Some(systeml2_vec) = systeml2_vec {
                    let exist_tag_set = self.exist_document_system.entry(current_doc_id.unwrap()).or_insert(HashSet::new());
                    for systeml2 in systeml2_vec {
                        let systeml2_id = self.exist_system_l2.get(systeml2).unwrap().to_owned();
                        if exist_tag_set.contains(systeml2) == false {
                            self.document_system.push(DocumentSystem::new(current_doc_id.unwrap(), systeml2_id));
                            exist_tag_set.insert(systeml2.to_string());
                        }
                    }
                }

                if let Some(mfl2_vec) = mfl2_vec {
                    let exist_tag_set = self.exist_document_mf.entry(current_doc_id.unwrap()).or_insert(HashSet::new());
                    for mfl2 in mfl2_vec {
                        let mfl2_id = self.exist_mf_l2.get(mfl2).unwrap().to_owned();
                        if exist_tag_set.contains(mfl2) == false {
                            self.document_mf.push(DocumentMf::new(current_doc_id.unwrap(), mfl2_id));
                            exist_tag_set.insert(mfl2.to_string());
                        }
                    }
                }

                if let Some(projectl2_vec) = projectl2_vec {
                    let exist_tag_set = self.exist_document_project.entry(current_doc_id.unwrap()).or_insert(HashSet::new());
                    for projectl2 in projectl2_vec {
                        let projectl2_id = self.exist_project_l2.get(projectl2).unwrap().to_owned();
                        if exist_tag_set.contains(projectl2) == false {
                            self.document_project.push(DocumentProject::new(current_doc_id.unwrap(), projectl2_id));
                            exist_tag_set.insert(projectl2.to_string());
                        }
                    }
                }

                if let Some(aspice_vec) = aspice_vec {
                    let exist_aspice_set = self.exist_document_aspice.entry(current_doc_id.unwrap()).or_insert(HashSet::new());
                    for aspice in aspice_vec {
                        let aspice = Aspice::from_str(aspice).unwrap();
                        if exist_aspice_set.contains(&aspice) == false {
                            self.document_aspice.push(DocumentAspiceMapping::new(current_doc_id.unwrap(), aspice.clone()));
                            exist_aspice_set.insert(aspice.clone());
                        }
                    }
                }
            }
            
            // handle associate_requirement
            for row in range.rows().skip(1) {
                let name = row.get(0).and_then(|cell| cell.get_string()).unwrap();
                let associate_requirement = row.get(8).and_then(|cell| cell.get_string());
                self.documents.iter_mut().find(|doc| doc.name == name).map(|doc| {
                    if let Some(associate_requirement) = associate_requirement {
                        doc.associate_requirement = self.exist_documents.get(associate_requirement).unwrap().to_owned();
                    }
                });
            }
        }

        Ok(())
    }
}
