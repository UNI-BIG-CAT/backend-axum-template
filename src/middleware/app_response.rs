/**************************************************************************************************
 * 读取错误码yaml文件
 **************************************************************************************************/
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct ErrorMassges(HashMap<u32, String>);

fn load_error_codes() -> ErrorMassges {
    let path = Path::new("config").join("errcodes");
    let mut map: HashMap<u32, String> = HashMap::new();

    let entries = fs::read_dir(path).expect("failed to read error codes directory");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    // 读取文件内容
                    let content = fs::read_to_string(&path)
                        .unwrap_or_else(|_| panic!("failed to read file {:?}", path));

                    // 解析 YAML
                    let yaml_map: HashMap<u32, String> = serde_yaml::from_str(&content)
                        .unwrap_or_else(|e| panic!("failed to parse yaml {:?}: {}", path, e));
                    for (k, v) in yaml_map {
                        map.insert(k, v);
                    }
                }
            }
        }
    }
    ErrorMassges(map)
}

pub static ERROR_MAP: Lazy<ErrorMassges> = Lazy::new(|| load_error_codes());

fn get_err_msg(code: u32) -> String {
    ERROR_MAP
        .0
        .get(&code)
        .cloned()
        .unwrap_or_else(|| format!("Unknown error ({})", code))
}

/**************************************************************************************************
 * 自定义通用返回值
 **************************************************************************************************/
use axum::response::Response;
use axum::{Json, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    // 构造成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    // 构造错误响应
    pub fn error(code: u32) -> Self {
        Self {
            code,
            message: get_err_msg(code),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "code": self.code,
            "message": self.message,
            "data": self.data,
        }));
        body.into_response()
    }
}

impl<T> std::fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError: {} - {}", self.code, self.message)
    }
}
