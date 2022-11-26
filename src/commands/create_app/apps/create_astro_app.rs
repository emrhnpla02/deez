use std::io::Write;

use super::AppWithPath;

pub fn create_astro_app(app: AppWithPath) -> anyhow::Result<()> {
  let AppWithPath {
    app_name, app_path, ..
  } = app;

  let app_path = app_path.to_str().unwrap();

  create_package_json(&app_name, app_path)?;
  create_tsconfig(app_path)?;
  create_astroconfig(app_path)?;
  create_folders(&app_name, app_path)?;

  Ok(())
}

fn create_package_json(app_name: &str, app_path: &str) -> anyhow::Result<()> {
  let package_json_path = std::path::PathBuf::from(format!("{app_path}/package.json"));
  let mut package_json_file = std::fs::File::create(package_json_path)?;
  let content = format!(
    r#"{{
  "name": "{}",
  "type": "module",
  "version": "0.0.1",
  "private": true,
  "scripts": {{
    "dev": "astro dev",
    "start": "astro dev",
    "build": "astro build",
    "preview": "astro preview",
    "astro": "astro"
  }}
}}"#,
    app_name
  );

  package_json_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_tsconfig(app_path: &str) -> anyhow::Result<()> {
  let tsconfig_path = std::path::PathBuf::from(format!("{app_path}/tsconfig.json"));
  let mut tsconfig_file = std::fs::File::create(tsconfig_path)?;
  let content = r#"{
  "extends": "astro/tsconfigs/strict"
}"#;

  tsconfig_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_astroconfig(app_path: &str) -> anyhow::Result<()> {
  let astroconfig_path = std::path::PathBuf::from(format!("{app_path}/astro.config.mjs"));
  let mut astroconfig_file = std::fs::File::create(astroconfig_path)?;
  let content = r#"import { defineConfig } from "astro/config";

export default defineConfig({});"#;

  astroconfig_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_folders(app_name: &str, app_path: &str) -> anyhow::Result<()> {
  std::fs::create_dir_all(format!("{app_path}/public/"))?;
  std::fs::create_dir_all(format!("{app_path}/src/pages/"))?;
  std::fs::create_dir_all(format!("{app_path}/src/components/"))?;
  std::fs::create_dir_all(format!("{app_path}/src/layouts/"))?;

  let env_path = std::path::PathBuf::from(format!("{app_path}/src/env.d.ts"));
  let mut env_file = std::fs::File::create(env_path)?;

  let env_content = r#"/// <reference types="astro/client" />"#;

  env_file.write_all(env_content.as_bytes())?;

  let indexastro_path = std::path::PathBuf::from(format!("{app_path}/src/pages/index.astro"));
  let mut indexastro_file = std::fs::File::create(indexastro_path)?;

  let indexastro_content = r#"---
import Layout from "../layouts/Layout.astro";
---

<Layout></Layout>"#;

  indexastro_file.write_all(indexastro_content.as_bytes())?;

  let layoutastro_path = std::path::PathBuf::from(format!("{app_path}/src/layouts/Layout.astro"));
  let mut layoutastro_file = std::fs::File::create(layoutastro_path)?;

  let layoutastro_content = format!(
    r#"---
---

<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width" />
    <meta name="generator" content={{Astro.generator}} />
    <title>{}</title>
  </head>
  <body>
    <slot />
  </body>
</html>"#,
    app_name
  );

  layoutastro_file.write_all(layoutastro_content.as_bytes())?;
  Ok(())
}
