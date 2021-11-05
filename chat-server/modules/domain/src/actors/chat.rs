use actix::{Actor, AsyncContext, Context, Handler, Message, Recipient};
use actix_broker::BrokerIssue;

use crate::chat::comment::Comment;

pub struct Chat {
    room_id: String,
    room: Recipient<BroadCastChatComment>,
}
// NOTE: T: Handler<Message1> + Handler<Message2> 的なことできるのかな？

impl Chat {
    pub fn new(room_id: String, room: Recipient<BroadCastChatComment>) -> Self {
        Chat {
            room_id,
            room,
        }
    }
}

impl Actor for Chat {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.issue_system_async(
            SubscribeChatComment {
                room_id: self.room_id.clone(),
                chat: ctx.address().recipient(),
            }
        );
    }
}


// ----- Messages & Handlers

pub struct NewChatCommentPosted {
    pub audience_id: String,
    pub text: String,
}

impl Message for NewChatCommentPosted {
    type Result = ();
}

impl Handler<NewChatCommentPosted> for Chat {
    type Result = ();

    fn handle(&mut self, msg: NewChatCommentPosted, _: &mut Self::Context) -> Self::Result {
        let comment =
            Comment::new(
                self.room_id.clone(),
                msg.audience_id.clone(),
                msg.text.clone(),
            );

        self.issue_system_async(SaveChatComment { comment });
    }
}

pub struct BroadCastChatComment {
    pub comment: Comment,
}

impl Message for BroadCastChatComment {
    type Result = ();
}

#[derive(Clone)]
pub struct SaveChatComment {
    pub comment: Comment,
}

impl Message for SaveChatComment {
    type Result = ();
}

pub struct ChatCommentSaved {
    pub comment: Comment,
}

impl Message for ChatCommentSaved {
    type Result = ();
}

impl Handler<ChatCommentSaved> for Chat {
    type Result = ();

    fn handle(&mut self, msg: ChatCommentSaved, _: &mut Self::Context) -> Self::Result {
        self.room.do_send(
            BroadCastChatComment { comment: msg.comment }
        ).unwrap();
    }
}

#[derive(Clone)]
pub struct SubscribeChatComment {
    pub room_id: String,
    pub chat: Recipient<ChatCommentSaved>,
}

impl Message for SubscribeChatComment {
    type Result = ();
}
