pub mod create_astro_app;
pub mod create_next_app;
pub mod create_react_app;
pub use create_astro_app::create_astro_app;
pub use create_next_app::create_next_app;
pub use create_react_app::create_react_app;

use crate::cli::AppType;

pub struct AppWithPath {
  pub app_type: AppType,
  pub app_name: String,
  pub app_path: std::path::PathBuf,
}
