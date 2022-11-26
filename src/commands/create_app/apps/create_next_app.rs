use std::io::Write;

use super::AppWithPath;

pub fn create_next_app(app: AppWithPath) -> anyhow::Result<()> {
  let AppWithPath {
    app_name, app_path, ..
  } = app;

  let app_path = app_path.to_str().unwrap();

  create_package_json(&app_name, app_path)?;
  create_tsconfig(app_path)?;
  create_nextconfig(app_path)?;
  create_nextenv(app_path)?;
  create_folders(&app_name, app_path)?;

  Ok(())
}

fn create_package_json(app_name: &str, app_path: &str) -> anyhow::Result<()> {
  let package_json_path = std::path::PathBuf::from(format!("{app_path}/package.json"));
  let mut package_json_file = std::fs::File::create(package_json_path)?;
  let content = format!(
    r#"{{
  "name": "{}",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
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
    "target": "es5",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["components/*"],
      "@utils/*": ["utils/*"],
      "@public/*": ["public/*"],
      "@styles/*": ["styles/*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx"],
  "exclude": ["node_modules"]
}"#;

  tsconfig_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_nextconfig(app_path: &str) -> anyhow::Result<()> {
  let nextconfig_path = std::path::PathBuf::from(format!("{app_path}/next.config.js"));
  let mut nextconfig_file = std::fs::File::create(nextconfig_path)?;
  let content = r#"/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
}

module.exports = nextConfig"#;

  nextconfig_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_nextenv(app_path: &str) -> anyhow::Result<()> {
  let nextenv_path = std::path::PathBuf::from(format!("{app_path}/next-env.d.ts"));
  let mut nextenv_file = std::fs::File::create(nextenv_path)?;
  let content = r#"/// <reference types="next" />
/// <reference types="next/image-types/global" />"#;

  nextenv_file.write_all(content.as_bytes())?;

  Ok(())
}

fn create_folders(app_name: &str, app_path: &str) -> anyhow::Result<()> {
  std::fs::create_dir_all(format!("{app_path}/pages/api/"))?;
  std::fs::create_dir_all(format!("{app_path}/public/"))?;
  std::fs::create_dir_all(format!("{app_path}/components/"))?;
  std::fs::create_dir_all(format!("{app_path}/styles/"))?;
  let globalscss_path = std::path::PathBuf::from(format!("{app_path}/styles/globals.css"));
  std::fs::File::create(globalscss_path)?;

  let apptsx_path = std::path::PathBuf::from(format!("{app_path}/pages/_app.tsx"));
  let mut apptsx_file = std::fs::File::create(apptsx_path)?;

  let apptsx_content = r#"import type { AppProps } from "next/app";
import Layout from "../components/Layout";
import "@styles/globals.css";

const App = ({ Component, pageProps }: AppProps) => {
  return (
    <Layout>
      <Component {...pageProps} />
    </Layout>
  );
};

export default App;"#;

  apptsx_file.write_all(apptsx_content.as_bytes())?;

  let indextsx_path = std::path::PathBuf::from(format!("{app_path}/pages/index.tsx"));
  let mut indextsx_file = std::fs::File::create(indextsx_path)?;

  let indextsx_content = format!(
    r#"import Head from "next/head";

const Home = () => {{
  return (
    <div>
      <Head>
        <title>{}</title>
      </Head>
    </div>
  );
}};

export default Home;"#,
    app_name
  );

  indextsx_file.write_all(indextsx_content.as_bytes())?;

  let layouttsx_path = std::path::PathBuf::from(format!("{app_path}/components/Layout.tsx"));
  let mut layouttsx_file = std::fs::File::create(layouttsx_path)?;

  let layouttsx_content = r#"import type { FC, ReactNode } from "react";

interface IProps {
  children: ReactNode;
}

const Layout: FC<IProps> = ({ children }) => <main>{children}</main>;

export default Layout;"#;

  layouttsx_file.write_all(layouttsx_content.as_bytes())?;

  Ok(())
}
