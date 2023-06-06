use url::Url;

use crate::{IndexClientResult, UploadResponse};

#[derive(Debug, Clone)]
pub struct IndexClient {
    base_url: Url,
}

impl IndexClient {
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }

    pub async fn upload_module(&self, module_name: &str, module_version: &str, module_bytes: Vec<u8>) -> IndexClientResult<UploadResponse> {
        let module_upload_part = reqwest::multipart::Part::bytes(module_bytes)
            .file_name(format!("{module_name}_{module_version}.tar"));

        let upload_url = self.base_url.join("/modules")?;
        let upload_response = reqwest::Client::new()
            .post(upload_url)
            .multipart(reqwest::multipart::Form::new().part("module bundle", module_upload_part))
            .send()
            .await?;

        Ok(upload_response.json::<UploadResponse>().await?)
    }
}