// @Author: BlahGeek
// @Date:   2017-06-17
// @Last Modified by:   BlahGeek
// @Last Modified time: 2020-01-17

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use dirs;

use crate::{
    actions::utils::open,
    mcore::{
        action::{Action, ActionResult},
        config::Config,
        item::{Icon, Item},
    },
};

struct FileBrowserEntry {
    name: String,
    path: PathBuf,
    is_file: bool,
}

impl FileBrowserEntry {
    fn new(name: String, path: PathBuf) -> Option<FileBrowserEntry> {
        if !(path.is_dir() || path.is_file()) {
            warn!("Invalid path: {:?}", path);
            None
        } else {
            let is_file = path.is_file();
            Some(FileBrowserEntry {
                name,
                path,
                is_file,
            })
        }
    }

    fn into_item(self) -> Item {
        Item {
            title: self.name.clone(),
            subtitle: Some(self.path.to_string_lossy().into()),
            badge: if self.is_file {
                Some("File".into())
            } else {
                Some("Directory".into())
            },
            icon: Some(if self.is_file {
                Icon::FontAwesome("file".into())
            } else {
                Icon::FontAwesome("folder".into())
            }),
            data: Some(self.path.to_string_lossy().into()),
            priority: -10,
            action: Some(Arc::new(self)),
            ..Item::default()
        }
    }
}

impl Action for FileBrowserEntry {
    fn runnable_bare(&self) -> bool {
        true
    }

    fn run_bare(&self) -> ActionResult {
        if self.is_file {
            open::that(&self.path.to_string_lossy())?;
            Ok(Vec::new())
        } else {
            let mut ret = Vec::new();

            debug!("Reading dir: {:?}", self.path);
            let entries = self.path.read_dir()?;
            for entry in entries.into_iter() {
                match entry {
                    Ok(entry) => {
                        if let Some(act) = FileBrowserEntry::new(
                            entry.file_name().to_string_lossy().into(),
                            entry.path(),
                        ) {
                            ret.push(act.into_item())
                        }
                    }
                    Err(error) => {
                        warn!("Read dir error: {}", error);
                    }
                }
            }
            if let Some(parent) = self.path.parent() {
                if let Some(act) = FileBrowserEntry::new("..".into(), parent.into()) {
                    let mut item = act.into_item();
                    item.priority = -100;
                    ret.push(item);
                }
            }
            Ok(ret)
        }
    }
}

#[derive(Deserialize)]
struct EntryConfig {
    name: String,
    path: String,
}

pub fn get(config: &Config) -> Vec<Item> {
    let entries = config
        .get::<Vec<EntryConfig>>(&["file_browser", "entries"])
        .unwrap();

    entries
        .into_iter()
        .map(|c| {
            let mut p = Path::new(&c.path).to_path_buf();
            if c.path.starts_with("~/") {
                if let Some(homedir) = dirs::home_dir() {
                    p = homedir;
                    p.push(Path::new(&c.path[2..]));
                }
            }
            FileBrowserEntry::new(c.name, p)
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().into_item())
        .collect()
}
