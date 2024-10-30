use bayou_protocol::{cryptography::key::Algorithms, types::activitystream_objects::actors::Actor};
use deadpool_postgres::Pool;
use uuid::Uuid;

use crate::db::{
    conn::{Conn, DbErr, EntityOrigin},
    utility::{instance_actor::InstanceActor, new_actor::NewLocal},
};

use super::{actors, create_user, init, instance_actor};

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

#[allow(unused_variables)]
impl Conn for PgConn {
    async fn init(&self, primary_domain: &str) -> Result<(), String> {
        init::init(self, primary_domain).await
    }

    async fn get_instance_actor(&self, algorithm: Algorithms) -> InstanceActor {
        instance_actor::get_instance_actor(self, algorithm).await
    }

    async fn create_user(&self, domain: &str, content: &NewLocal) -> Result<Uuid, DbErr> {
        create_user::create_user(self, domain, content).await
    }

    async fn get_actor(&self, username: &str, origin: &EntityOrigin<'_>) -> Option<Actor> {
        actors::get_actor(self, username, origin).await
    }

    async fn backfill_actor(&self, username: &str, origin: &EntityOrigin<'_>) -> Option<Actor> {
        todo!()
    }
}
