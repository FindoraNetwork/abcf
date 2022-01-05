// #![feature(inherent_associated_types)]

use std::path::PathBuf;

mod error;
pub use error::{Error, Result};
use tm_abci::Application;

pub struct Node<A: Application> {
    pub app: A,
    path: PathBuf,
}

impl<A> Node<A>
where
    A: Application + 'static,
{
    pub fn new(app: A, home: &str) -> Result<Self> {
        let path = PathBuf::from(home);
        if !path.exists() {
            // FIXME: it was not implement well, the second arg NodeType was not passing into
            // init_home function
            tendermint_sys::init_home(home, tendermint_sys::NodeType::FullNode)?;
        }

        Ok(Self { app, path })
    }

    pub fn start(self) -> Result<tendermint_sys::Node> {
        let path_buf = self.path.join("config/config.toml");
        let path = path_buf.to_str().ok_or(Error::PathError)?;
        let td_node = tendermint_sys::Node::new(path, self.app)?;
        td_node.start()?;
        Ok(td_node)
    }
}

// impl<A> Node<A>
// where
//     A: Application,
// {
//     type Target = A;
//
//     fn deref(&self) -> &A {
//         &self.app
//     }
// }
//
// impl<A> Node<A>
// where
//     A: Application,
// {
//     fn deref_mut(&mut self) -> &mut A {
//         &mut self.app
//     }
// }
