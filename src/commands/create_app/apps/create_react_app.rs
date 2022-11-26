use std::io::Write;

use super::AppWithPath;

pub fn create_react_app(app: AppWithPath) -> anyhow::Result<()> {
  let AppWithPath {
    app_name, app_path, ..
  } = app;

  let app_path = app_path.to_str().unwrap();

  create_package_json(&app_name, app_path)?;
  create_tsconfig(app_path)?;
  create_viteconfig(app_path)?;
  create_indexhtml(&app_name, app_path)?;
  create_folders(app_path)?;

  Ok(())
}

fn create_package_json(app_name: &str, app_path: &str) -> anyhow::Result<()> {
  let package_json_path = std::path::PathBuf::from(format!("{app_path}/package.json"));
  let mut package_json_file = std::fs::File::create(package_json_path)?;
  let content = format!(
    r#"{{
  "name": "{}",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {{
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
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
  "compilerOptions": {
    "allowJs": false,
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": false,
    "forceConsistentCasingInFileNames": true,
    "isolatedModules": true,
    "jsx": "react-jsx",
    "lib": ["DOM", "DOM.Iterable", "ESNext"],
    "module": "ESNext",
    "moduleResolution": "Node",
    "noEmit": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "strict": true,
    "target": "ESNext",
    "useDefineForClassFields": true
  },
  "include": ["src"],
  "references": [
    {
      "path": "./tsconfig.node.json"
    }
  ]
  }"#;

  tsconfig_file.write_all(content.as_bytes())?;

  let tsconfig_node_path = std::path::PathBuf::from(format!("{app_path}/tsconfig.node.json"));
  let mut tsconfig_node_file = std::fs::File::create(tsconfig_node_path)?;
  let content = r#"{
  "compilerOptions": {
    "allowSyntheticDefaultImports": true,
    "composite": true,
    "module": "ESNext",
    "moduleResolution": "Node"
  },
  "include": ["vite.config.ts"]
  }"#;

  tsconfig_node_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_viteconfig(app_path: &str) -> anyhow::Result<()> {
  let viteconfig_path = std::path::PathBuf::from(format!("{app_path}/vite.config.ts"));
  let mut viteconfig_file = std::fs::File::create(viteconfig_path)?;
  let content = r#"import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
});"#;

  viteconfig_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_indexhtml(app_name: &str, app_path: &str) -> anyhow::Result<()> {
  let indexhtml_path = std::path::PathBuf::from(format!("{app_path}/index.html"));
  let mut indexhtml_file = std::fs::File::create(indexhtml_path)?;

  let content = format!(
    r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{app_name}</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>"#
  );

  indexhtml_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_folders(app_path: &str) -> anyhow::Result<()> {
  std::fs::create_dir_all(format!("{app_path}/public/"))?;
  std::fs::create_dir_all(format!("{app_path}/src/assets/"))?;
  std::fs::create_dir_all(format!("{app_path}/src/components/"))?;

  let viteenv_path = std::path::PathBuf::from(format!("{app_path}/src/vite-env.d.ts"));
  let mut viteenv_file = std::fs::File::create(viteenv_path)?;

  let viteenv_content = r#"/// <reference types="vite/client" />"#;

  viteenv_file.write_all(viteenv_content.as_bytes())?;

  let maintsx_path = std::path::PathBuf::from(format!("{app_path}/src/main.tsx"));
  let mut maintsx_file = std::fs::File::create(maintsx_path)?;

  let maintsx_content = r#"import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);"#;

  maintsx_file.write_all(maintsx_content.as_bytes())?;

  let apptsx_path = std::path::PathBuf::from(format!("{app_path}/src/App.tsx"));
  let mut apptsx_file = std::fs::File::create(apptsx_path)?;

  let apptsx_content = r#"import type { FC } from "react";

const App: FC = () => {
  return <main></main>;
};

export default App;"#;

  apptsx_file.write_all(apptsx_content.as_bytes())?;

  Ok(())
}
