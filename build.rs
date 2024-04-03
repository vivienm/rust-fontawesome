use std::{env, fs::File, path::PathBuf};

use anyhow::Result;
use convert_case::{Case, Casing};
use serde::Serialize;
use tera::Tera;

mod api {
    use std::collections::HashMap;

    use anyhow::Result;
    use serde::Deserialize;

    static URL: &str = "https://api.fontawesome.com";
    static QUERY_PARAM: &str = "\
        query {
            release (version: \"6.5.2\") {
                version,
                icons {
                    id,
                    label,
                    unicode,
                    membership {
                        free,
                    },
                },
            },
        },
    ";

    #[derive(Deserialize)]
    struct Response {
        data: Data,
    }

    #[derive(Deserialize)]
    struct Data {
        release: Release,
    }

    #[derive(Debug, Deserialize)]
    pub struct Release {
        pub icons: Vec<Icon>,
        pub version: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Icon {
        pub id: String,
        pub label: String,
        pub membership: Membership,
        pub unicode: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Membership {
        pub free: Vec<String>,
    }

    pub async fn get_release() -> Result<Release> {
        let client = reqwest::Client::new();
        let resp: Response = client
            .post(URL)
            .json(&HashMap::from([("query", QUERY_PARAM)]))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data.release)
    }
}

#[derive(Serialize)]
struct Icon {
    ident: String,
    char_ident: String,
    name: String,
    label: String,
    is_free: bool,
    unicode: String,
}

impl Icon {
    fn escape_leading_digit(mut name: String) -> String {
        if name
            .chars()
            .next()
            .expect("name should not be empty")
            .is_ascii_digit()
        {
            name.insert(0, '_');
        }
        name
    }

    fn ident_from_name(name: &str) -> String {
        Self::escape_leading_digit(name.to_case(Case::Pascal))
    }

    fn char_ident_from_name(name: &str) -> String {
        Self::escape_leading_digit(name.to_case(Case::ScreamingSnake))
    }
}

impl From<api::Icon> for Icon {
    fn from(icon: api::Icon) -> Self {
        Self {
            ident: Self::ident_from_name(&icon.id),
            char_ident: Self::char_ident_from_name(&icon.id),
            name: icon.id,
            label: icon.label,
            is_free: !icon.membership.free.is_empty(),
            unicode: icon.unicode,
        }
    }
}

type Version = (u8, u8, u8);

fn parse_version(s: &str) -> Result<Version> {
    let mut components = s.split('.');
    let major: u8 = components.next().expect("major").parse()?;
    let minor: u8 = components.next().expect("minor").parse()?;
    let patch: u8 = components.next().expect("patch").parse()?;
    assert!(components.next().is_none());
    Ok((major, minor, patch))
}

struct Release {
    version: Version,
    icons: Vec<Icon>,
}

async fn get_release() -> Result<Release> {
    let api_release = api::get_release().await?;
    Ok(Release {
        version: parse_version(&api_release.version)?,
        icons: api_release.icons.into_iter().map(Into::into).collect(),
    })
}

fn render_template(
    templates: &Tera,
    context: &tera::Context,
    template_name: &str,
    target_name: &str,
) -> Result<()> {
    let target_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let target_path = target_dir.join(target_name);

    let target_file = File::create(&target_path)?;
    templates.render_to(template_name, context, target_file)?;
    println!("Output file written to {:?}", &target_path);
    println!("cargo:rerun-if-changed=src/{template_name}");
    Ok(())
}

fn render_templates(release: &Release) -> Result<()> {
    let mut source_dir: PathBuf = env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    source_dir.push("src");
    let templates = Tera::new(source_dir.join("*.tera").to_str().unwrap())?;

    let mut context = tera::Context::new();
    context.insert("version", &release.version);
    context.insert("icons", &release.icons);

    render_template(&templates, &context, "icon.rs.tera", "icon.rs")?;
    render_template(&templates, &context, "version.rs.tera", "version.rs")?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let release = get_release().await?;
    println!("cargo:rerun-if-changed=build.rs");
    render_templates(&release)?;
    Ok(())
}
