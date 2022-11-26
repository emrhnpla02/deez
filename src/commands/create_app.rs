mod apps;
mod integrations;

use std::io::{Read, Write};
use std::str::FromStr;

use self::apps::AppWithPath;
use crate::cli::{App, AppType};
use crate::configs::CreateAppConfig;

const DEEZ_CREATE_APP_CONFIG: &str = "deez_create_app_config";

pub async fn create_app(app: App) -> anyhow::Result<()> {
  let App {
    app_name,
    app_type,
    config,
  } = app;

  let app_path = create_app_dir(&app_name)?;
  let packages = determine_packages(&app_type);
  let dev_packages = determine_dev_packages(&app_type);
  handle_config(
    &app_path,
    &packages,
    &dev_packages.clone().unwrap_or_default(),
    &config,
  )?;
  let app_with_path = AppWithPath {
    app_path: app_path.clone(),
    app_name,
    app_type: app_type.clone(),
  };
  create_app_structure(app_with_path)?;
  handle_integrations(app_path.clone(), &app_type)?;
  handle_packages(packages, dev_packages, app_path).await?;

  Ok(())
}

fn create_app_dir(app_name: &str) -> anyhow::Result<std::path::PathBuf> {
  let dir = format!("./{app_name}");
  let dir = std::path::Path::new(dir.as_str());

  std::fs::create_dir_all(dir)?;
  Ok(dir.canonicalize()?)
}

fn determine_packages(app_type: &AppType) -> Vec<&str> {
  match app_type {
    AppType::React => vec!["react", "react-dom"],
    AppType::Next => vec!["react", "react-dom", "next"],
    AppType::Astro => vec!["astro"],
  }
}

fn determine_dev_packages(app_type: &AppType) -> Option<Vec<&str>> {
  match app_type {
    AppType::React => Some(vec![
      "vite",
      "typescript",
      "@types/react",
      "@types/react-dom",
      "@vitejs/plugin-react",
    ]),
    AppType::Next => Some(vec![
      "typescript",
      "@types/node",
      "@types/react",
      "@types/react-dom",
    ]),
    AppType::Astro => None,
  }
}

fn handle_config(
  app_path: &std::path::Path,
  packages: &Vec<&str>,
  dev_packages: &Vec<&str>,
  config: &Option<std::path::PathBuf>,
) -> anyhow::Result<()> {
  use crate::utils::get_extension_from_filename;
  use anyhow::bail;

  if let Some(config_path) = config {
    if get_extension_from_filename(config_path.to_str().unwrap()) != Some("toml") {
      bail!("Configuration file must be a toml file");
    };

    read_and_store_config(config_path)?;
  } else {
    let app_path = app_path.to_str().unwrap();
    let config_path = std::path::PathBuf::from(format!("{app_path}/.deez.toml"));
    let mut config_file = std::fs::File::create(&config_path)?;

    config_file.write_all(
      format!(
        r"app_name = 'DeezNuts'
package_manager = 'pnpm'
packages = {packages:?}
dev_packages  = {dev_packages:?}
integrations = []"
      )
      .as_bytes(),
    )?;

    edit::edit_file(&config_path)?;

    read_and_store_config(&config_path)?;
    std::fs::remove_file(config_path)?;
  };

  Ok(())
}

fn read_and_store_config(config_file: &std::path::PathBuf) -> anyhow::Result<()> {
  let mut content = String::new();

  std::fs::File::open(&config_file)?.read_to_string(&mut content)?;
  let config = toml::from_str::<CreateAppConfig>(&content)?;

  confy::store(DEEZ_CREATE_APP_CONFIG, None, &config)?;

  Ok(())
}

fn create_app_structure(app: AppWithPath) -> anyhow::Result<()> {
  use self::apps::{create_astro_app, create_next_app, create_react_app};
  match app.app_type {
    AppType::React => create_react_app(app)?,
    AppType::Next => create_next_app(app)?,
    AppType::Astro => create_astro_app(app)?,
  };

  Ok(())
}

fn handle_integrations(app_path: std::path::PathBuf, app_type: &AppType) -> anyhow::Result<()> {
  use self::integrations::{add_git, add_tailwind};
  use crate::configs::create_app_config::Integrations;

  let cfg: CreateAppConfig = confy::load(DEEZ_CREATE_APP_CONFIG, None)?;

  if let Some(integrations) = cfg.integrations {
    for i in integrations {
      match i {
        Integrations::Git => add_git(&app_path, app_type)?,
        Integrations::Tailwind => add_tailwind()?,
      }
    }
  }

  Ok(())
}

async fn handle_packages(
  packages: Vec<&str>,
  dev_packages: Option<Vec<&str>>,
  path: std::path::PathBuf,
) -> anyhow::Result<()> {
  let path = path.to_str().unwrap();
  let packages = packages.as_slice();

  println!("Installing dependencies..");
  install_packages(packages, path, false).await?;

  if let Some(dev_packages) = dev_packages {
    println!("Installing dev dependencies..");
    install_packages(dev_packages.as_slice(), path, true).await?;
  }

  Ok(())
}

async fn install_packages(
  packages: &[&str],
  path: &str,
  as_dev_dependency: bool,
) -> anyhow::Result<()> {
  use futures_lite::{io::BufReader, prelude::*};

  let cfg: CreateAppConfig = confy::load(DEEZ_CREATE_APP_CONFIG, None)?;
  let manager = kayra::Manager::from_str(cfg.package_manager.as_str())?;

  let mut child = {
    if as_dev_dependency {
      kayra::PackageManager::new(manager)
        .dir(path)
        .flags(&["-D"])
        .install(packages)
        .async_run()
        .await?
    } else {
      kayra::PackageManager::new(manager)
        .dir(path)
        .install(packages)
        .async_run()
        .await?
    }
  };

  let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

  while let Some(line) = lines.next().await {
    println!("{}", line?);
  }

  Ok(())
}
