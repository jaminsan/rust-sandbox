use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message, MessageResult};

use crate::actors::room::Room;

// NOTE: 正直こいつは外側のレイヤーにおいても良いかもしれない
pub struct Entrance {
    rooms: HashMap<String, Addr<Room>>,
}

impl Default for Entrance {
    fn default() -> Self {
        Entrance {
            rooms: HashMap::new()
        }
    }
}

impl Actor for Entrance {
    type Context = Context<Self>;
}


// ----- Messages & Handlers

pub struct AudienceArrived {
    pub room_id: String,
}

impl Message for AudienceArrived {
    type Result = Addr<Room>;
}

impl Handler<AudienceArrived> for Entrance {
    type Result = MessageResult<AudienceArrived>;

    fn handle(&mut self, msg: AudienceArrived, _: &mut Self::Context) -> Self::Result {
        let rooms = &mut self.rooms;
        let room_id = msg.room_id.clone();
        let room =
            match rooms.get(&room_id) {
                None => {
                    let room = Room::start(room_id.clone());
                    self.rooms.insert(room_id.clone(), room.clone());
                    room
                }

                Some(room) => {
                    room.clone()
                }
            };

        MessageResult(room)
    }
}
