use std::io::Write;

use crate::cli::AppType;

pub fn add_git(path: &std::path::PathBuf, app_type: &AppType) -> anyhow::Result<()> {
  std::process::Command::new("git")
    .arg("init")
    .current_dir(path)
    .spawn()?;

  let path = path.to_str().unwrap();
  let gitignore_path = std::path::PathBuf::from(format!("{path}/.gitignore"));
  let mut gitignore_file = std::fs::File::create(gitignore_path)?;
  let content = determine_content(app_type);

  gitignore_file.write_all(content.as_bytes())?;

  Ok(())
}

fn determine_content(app_type: &AppType) -> String {
  match app_type {
    AppType::React => r"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?"
      .to_owned(),
    AppType::Next => r"# dependencies
/node_modules
/.pnp
.pnp.js

# testing
/coverage

# next.js
/.next/
/out/

# production
/build

# misc
.DS_Store
*.pem

# debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*

# local env files
.env*.local

# vercel
.vercel

# typescript
*.tsbuildinfo
next-env.d.ts"
      .to_owned(),
    AppType::Astro => r"# build output
dist/

# dependencies
node_modules/

# logs
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*


# environment variables
.env
.env.production

# macOS-specific files
.DS_Store"
      .to_owned(),
  }
}
