use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppConfig {
  pub app_name: String,
  pub package_manager: String,
  pub packages: Option<Vec<String>>,
  pub dev_packages: Option<Vec<String>>,
  pub integrations: Option<Vec<Integrations>>,
}

impl Default for CreateAppConfig {
  fn default() -> Self {
    Self {
      app_name: "DeezNuts".to_owned(),
      package_manager: "pnpm".to_owned(),
      packages: None,
      dev_packages: None,
      integrations: None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Integrations {
  Git,
  Tailwind,
}
