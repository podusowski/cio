use anyhow::Context;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ImageOrBuild {
    /// Use this image tag.
    Image { image: String },
    /// Build docker image from context given in `build`.
    Build { build: String },
    /// Similar to build, but Dockerfile content is defined in `recipe`.
    Recipe { recipe: String },
}

#[derive(Debug, Deserialize)]
pub struct Mission {
    #[serde(flatten)]
    pub image_or_build: ImageOrBuild,
    #[serde(flatten)]
    pub container_options: ContainerOptions,
    pub script: String,
}

#[derive(Debug, Deserialize)]
pub struct Shell {
    #[serde(flatten)]
    pub image_or_build: ImageOrBuild,
    #[serde(flatten)]
    pub container_options: ContainerOptions,
}

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub missions: HashMap<String, Mission>,
    pub shell: Option<Shell>,
}

impl Plan {
    /// Load the plan from a YAML file.
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        Self::_from_file(path).with_context(|| format!("could not load '{}'", path))
    }

    fn _from_file(path: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
    }
}

fn false_() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct ContainerOptions {
    /// Work as current user in the container instead of default one (typically root).
    #[serde(default = "false_")]
    pub current_user: bool,
}
