use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, MessageResult, WeakAddr, WeakRecipient};

use crate::actors::audience::{Audience, NewChatCommentReceived, Reflect};
use crate::actors::chat::{BroadCastChatComment, Chat};

pub struct Room {
    room_id: String,
    chat: Addr<Chat>,
    audiences: Audiences,
}

impl Room {
    pub fn start(room_id: String) -> Addr<Self> {
        Room::create(move |ctx| {
            let addr = ctx.address();

            let chat =
                Chat::new(
                    room_id.clone(),
                    addr.recipient(),
                ).start();

            Self::new(room_id, chat)
        })
    }

    fn new(room_id: String, chat: Addr<Chat>) -> Self {
        Room {
            room_id,
            chat,
            audiences: Audiences::default(),
        }
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}

pub struct Audiences(pub Vec<WeakAddr<Audience>>);

impl Default for Audiences {
    fn default() -> Self {
        Audiences(vec![])
    }
}

impl Audiences {
    fn gc(&mut self) {
        self.0 =
            self.0
                .drain(..)
                .filter(|addr| addr.upgrade().is_some())
                .collect();
    }

    fn add(&mut self, addr: WeakAddr<Audience>) {
        self.0.push(addr);
    }
}


// ----- Messages  & Handlers -----

impl Handler<BroadCastChatComment> for Room {
    type Result = ();

    fn handle(&mut self, message: BroadCastChatComment, _: &mut Self::Context) -> Self::Result {
        for audience in self.audiences.0.iter() {
            if let Some(addr) = audience.upgrade() {
                addr.do_send(
                    NewChatCommentReceived {
                        comment: message.comment.clone()
                    })
            }
        }

        self.audiences.gc();
    }
}

pub struct AudienceJoined {
    pub audience_id: String,
    pub audience_tx: WeakRecipient<Reflect>,
}

impl Message for AudienceJoined {
    type Result = Addr<Audience>;
}

impl Handler<AudienceJoined> for Room {
    type Result = MessageResult<AudienceJoined>;

    fn handle(&mut self, msg: AudienceJoined, _: &mut Self::Context) -> Self::Result {
        let audience =
            Audience::new(
                msg.audience_id,
                msg.audience_tx,
                self.chat.clone(),
            ).start();

        self.audiences.add(audience.downgrade());
        println!(">>>>> Total number of audiences: {}", self.audiences.0.len());

        MessageResult(audience)
    }
}
