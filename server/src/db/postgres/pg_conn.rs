use bayou_protocol::cryptography::key::Algorithms;
use deadpool_postgres::Pool;

use crate::db::{conn::Conn, utility::{instance_actor::InstanceActor, new_actor::NewLocal}};

use super::{init, instance_actor};

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

#[allow(unused_variables)]
impl Conn for PgConn {
    async fn init(&self) -> Result<(), String> {
        init::init(self).await
    }

    async fn get_instance_actor(&self, algorithm: Algorithms) -> InstanceActor {
        instance_actor::get_instance_actor(self, algorithm).await
    }
    
    async fn create_user(&self,domain: &str,content: &NewLocal) -> Result<String,()> {
        todo!()
    }
}
