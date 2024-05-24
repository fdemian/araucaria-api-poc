pub mod key_value_storage {
    pub async fn store_value(key: &str, value: &str) -> serde_json::Value {
        return serde_json::json!({
        "ok": true,
        "key": key,
        "value": value
        });
    }
}

pub mod file_storage {
    pub async fn download_file() -> serde_json::Value {
        return serde_json::json!({
            "status": 200,
            "path": "/user/path/to/file.json",
            "filename": "file.json",
            "size": 224
        });
    }
}
