use uuid::Uuid;

use crate::db::{pg_sesh::Sesh, types::room::Room};

#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_room(&self, room: Room) -> Room {
        // let id = Uuid::now_v7();
        let result = self
            .query(
                Room::create_statement(),
                &[
                    &room.id,
                    &room.external_id,
                    &room.domain,
                    &room.community,
                    &room.system_channel,
                    &room.created,
                    &room.known_complete,
                    &room.is_dm,
                    &room.user_a,
                    &room.user_b,
                    &room.info.name,
                    &room.info.description,
                    &room.info.category,
                    &room.info.display_order,
                ],
            )
            .await
            .expect("failed to create room")
            .pop()
            .expect("creating room returned nothing");
        result.into()
    }
    pub async fn get_room(&self, room_id: &Uuid) -> Option<Room> {
        let result = self
            .query(Room::read_statement(), &[room_id])
            .await
            .expect("failed to fetch room")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn update_room(&self, room: Room) -> Room {
        let result = self
            .query(
                Room::update_statement(),
                &[
                    &room.external_id,
                    &room.domain,
                    &room.system_channel,
                    &room.info.name,
                    &room.info.description,
                    &room.known_complete,
                    &room.info.category,
                    &room.info.display_order,
                    &room.id,
                ],
            )
            .await
            .expect("failed to update room")
            .pop()
            .expect("updating room returned nothing");
        result.into()
    }
    pub async fn delete_room(&self, room_id: &Uuid) {
        let _result = self
            .query(Room::delete_statement(), &[room_id])
            .await
            .expect("failed to delete room");
    }
    pub async fn get_all_comm_rooms(&self, com_id: &Uuid) -> Vec<Room> {
        let result = self
            .query(Room::get_all_comm_rooms(), &[com_id])
            .await
            .expect("failed to fetch community rooms");
        result.into_iter().map(|x| x.into()).collect()
    }
}