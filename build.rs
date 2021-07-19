use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use tera::Tera;

static FA_VERSION: &str = "5.15.3";

#[derive(Debug, Deserialize)]
struct IconData {
    label: String,
    unicode: String,
}

#[derive(Debug, Serialize)]
struct Icon {
    const_ident: String,
    enum_ident: String,
    name: String,
    label: String,
    unicode: String,
}

fn fix_leading_digit(mut icon_name: String) -> String {
    if icon_name
        .chars()
        .next()
        .expect("name should not be empty")
        .is_digit(10)
    {
        icon_name.insert(0, '_');
    }
    icon_name
}

fn const_ident_from_name(icon_name: &str) -> String {
    icon_name.to_case(Case::ScreamingSnake)
}

fn enum_ident_from_name(icon_name: &str) -> String {
    fix_leading_digit(icon_name.to_case(Case::Pascal))
}

async fn load_icons() -> anyhow::Result<Vec<Icon>> {
    let fa_url = format!(
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/{}/metadata/icons.yml",
        FA_VERSION
    );
    let fa_data = reqwest::get(&fa_url).await?.bytes().await?;
    let icons: HashMap<String, IconData> = serde_yaml::from_slice(&fa_data)?;
    let mut icons: Vec<Icon> = icons
        .into_iter()
        .map(|(name, data)| Icon {
            const_ident: const_ident_from_name(&name),
            enum_ident: enum_ident_from_name(&name),
            name,
            label: data.label,
            unicode: data.unicode,
        })
        .collect();
    icons.sort_by(|i1, i2| i1.unicode.cmp(&i2.unicode));
    Ok(icons)
}

fn dump_module<P>(icons: &[Icon], target_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let mut source_dir: PathBuf = env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    source_dir.push("src");
    let templates = Tera::new(source_dir.join("*.tera").to_str().unwrap())?;
    let mut template_ctx = tera::Context::new();
    template_ctx.insert("version", &FA_VERSION);
    template_ctx.insert("icons", &icons);
    let target_file = File::create(&target_path)?;
    templates.render_to("icon.rs.tera", &template_ctx, target_file)?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let icons = load_icons().await?;

    let target_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let target_path = target_dir.join("icon.rs");
    dump_module(&icons, &target_path)?;

    println!("Output file written to {:?}", &target_path);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/icon.rs.tera");
    Ok(())
}
