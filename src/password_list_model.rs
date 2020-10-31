use vgtk::lib::gtk::*;
use id_tree::{Tree, Node, NodeId, InsertBehavior};
use directories::{BaseDirs};

use std::{io, fs, env};
use std::path::{Path, PathBuf};
use std::cmp::Ordering;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default)]
pub struct SharedPasswordListModel(Rc<PasswordListModel>);

impl Deref for SharedPasswordListModel {
    type Target = PasswordListModel;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for SharedPasswordListModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.0).expect("SharedPasswordListModel was borrowed mutably while it shouldn't!")
    }
}

#[derive(Clone, Debug)]
pub enum Entry {
    Password(String),
    Directory(String),
}

#[derive(Clone, Debug)]
pub struct PasswordListModel {
    pub passwords: Tree<Entry>,
    pub password_store_path: PathBuf,
    pub history: Vec<String>,
    pub current_password: Option<String>,
    pub show_main_menu: Option<MenuButton>,
    pub error: Option<String>,
}

impl Default for PasswordListModel {
    fn default() -> Self {
        let password_store_path = env::var("PASSWORD_STORE_DIR")
            .ok()
            .and_then(|password_store_dir| {
                let path = Path::new(&password_store_dir);
                if path.read_dir().is_ok() {
                    Some(path.to_owned())
                } else {
                    None
                }
            })
            .or(BaseDirs::new().map(|base_dirs| base_dirs.home_dir().join(".password-store")))
            .expect("Environment variable PASSWORD_STORE_DIR not set and/or no home directory found for this user!");

        let mut me = Self {
            passwords: Tree::new(),
            password_store_path,
            history: vec![".".into()],
            current_password: None,
            show_main_menu: None,
            error: None,
        };
        me.reload_passwords();

        me
    }
}

fn is_special_entry(path: &Path) -> bool {
    match path.file_name().unwrap_or("..".as_ref()).to_string_lossy().as_ref() {
        ".git" | ".gitattributes" | ".gpg-id" => true,
        _ => false,
    }
}

impl PasswordListModel {
    pub fn visible_page(&self) -> String {
        if self.current_password.is_some() {
            "show_password".into()
        } else {
            self.history.join("/")
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.history.len() > 1 || self.current_password.is_some()
    }

    pub fn is_active_page(&self, page: &str) -> bool {
        self.history.join("/") == page && self.current_password.is_none()
    }

    pub fn reload_passwords(&mut self) {
        self.passwords = Tree::new();
        let root_id = self.passwords
            .insert(Node::new(Entry::Directory(".".into())), InsertBehavior::AsRoot)
            .unwrap();

        if let Err(err) = self.load_passwords_from_dir(&self.password_store_path.clone(), &root_id) {
            self.error = Some(err.to_string());
        }
    }

    fn load_passwords_from_dir(&mut self, dir: &Path, parent: &NodeId) -> io::Result<()> {
        let mut read_dir = fs::read_dir(dir)?
            .filter(|dir_entry| dir_entry.is_ok())
            .map(|dir_entry| dir_entry.unwrap())
            .collect::<Vec<_>>();

        read_dir.sort_by(|a, b| {
            if a.path().is_dir() && !b.path().is_dir() {
                Ordering::Less
            } else if !a.path().is_dir() && b.path().is_dir() {
                Ordering::Greater
            } else {
                let a_stem = a.path()
                    .file_stem()
                    .unwrap_or("..".as_ref())
                    .to_string_lossy()
                    .to_lowercase();
                let b_stem = b.path()
                    .file_stem()
                    .unwrap_or("..".as_ref())
                    .to_string_lossy()
                    .to_lowercase();

                a_stem.cmp(&b_stem)
            }
        });

        for entry in read_dir.drain(..) {
            let path = entry.path();
            if path.is_dir() && !is_special_entry(&path) {
                let subdir = self.passwords.insert(
                    Node::new(Entry::Directory(
                        path.file_name()
                            .unwrap_or("..".as_ref())
                            .to_string_lossy()
                            .to_string()
                    )),
                    InsertBehavior::UnderNode(parent),
                ).unwrap();
                self.load_passwords_from_dir(&path, &subdir)?;
            } else if !is_special_entry(&path) {
                let _pw_id = self.passwords.insert(
                    Node::new(Entry::Password(
                        path.file_stem()
                            .unwrap_or("..".as_ref())
                            .to_string_lossy()
                            .to_string()
                    )),
                    InsertBehavior::UnderNode(parent),
                ).unwrap();
            }
        }

        Ok(())
    }
}
