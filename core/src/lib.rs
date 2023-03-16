mod handler;
mod id_gen;
mod models;
mod transport;

pub mod prelude {
    pub use crate::id_gen::IdGenerator;
    pub use crate::models::*;
    pub use crate::transport::*;
}

use crate::handler::Server;

pub fn run() {
    let mut server = Server::default();
    server.run();
}
