use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

mod error;
pub use error::{Error, Result};

pub struct Node<S: bs3::Store + 'static> {
    node: abcf::Node<S>,
    path: &'static Path,
}

impl<S: bs3::Store> Node<S> {
    pub fn new(home: &'static str) -> Result<Self> {
        let path = Path::new(home);
        if !path.exists() {
            tendermint_sys::init_home(home)?;
        }

        let node = abcf::Node::new();

        Ok(Self { node, path })
    }

    pub fn start(self) -> Result<tendermint_sys::Node> {
        let path_buf = self.path.join("config/config.toml");
        let path = path_buf.to_str().ok_or(Error::PathError)?;
        let td_node = tendermint_sys::Node::new(path, self.node)?;
        td_node.start()?;
        Ok(td_node)
    }
}

impl<S: bs3::Store> Deref for Node<S> {
    type Target = abcf::Node<S>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<S: bs3::Store> DerefMut for Node<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
