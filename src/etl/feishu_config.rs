use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    // [Post url] tenant_access_token post url
    pub tenant_access_token_url: String,
    // [Post body]tenant access token post payload
    pub app_id: String,
    pub app_secret: String,
    // [Post response] tenant_access_token response
    pub tenant_access_token: Option<String>,

    // [Post url] Create download task url
    pub create_export_task_url: String,
    // [Post body] The document to be downloaded
    pub doc: HashMap<String, String>,
    // [Post response] Onging download task ticket(id)
    pub export_task_ticket: Option<String>,

    // [Get url] Poll task result
    pub query_task_result_url: String,
    pub file_token: Option<String>,
    pub file_name: Option<String>,

    // [Get response] Binary file, save to local
    pub get_exported_file_url: String,
}

impl Config {
    pub fn new() -> Config {
        let mut cfg = Config {
            // Init tenant access token
            tenant_access_token_url: String::from(
                "https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal/",
            ),
            app_id: String::from("cli_a4537df102f81013"),
            app_secret: String::from("Alxjwxpwqg6KUWo28auGufKA385q5nzz"),
            tenant_access_token: None,

            // Init create export task
            create_export_task_url: String::from(
                "https://open.feishu.cn/open-apis/drive/v1/export_tasks",
            ),
            doc: HashMap::new(),
            export_task_ticket: None,

            // Init poll task status
            query_task_result_url: String::from(
                "https://open.feishu.cn/open-apis/drive/v1/export_tasks/:ticket",
            ),
            file_token: None,
            file_name: None,

            // Init download file
            get_exported_file_url: String::from(
                "https://open.feishu.cn/open-apis/drive/v1/export_tasks/file/:file_token/download",
            ),
        };

        cfg.doc.insert(
            String::from("token"),
            String::from("U6CmbMf18aGdG5sGOR2cXGC6nYc"),
        );
        cfg.doc
            .insert(String::from("file_extension"), String::from("xlsx"));
        cfg.doc
            .insert(String::from("type"), String::from("bitable"));

        cfg
    }
}
