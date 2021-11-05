use actix::{Actor, Context, Handler, Message, WeakRecipient};
use actix::prelude::*;

use crate::actors::chat::Chat;
use crate::chat::comment::Comment;

pub struct Audience {
    id: String,
    audience_tx: WeakRecipient<Reflect>,
    chat: Addr<Chat>,
}

impl Audience {
    pub fn new(audience_id: String, audience_tx: WeakRecipient<Reflect>, chat: Addr<Chat>) -> Self {
        Audience {
            id: audience_id,
            audience_tx,
            chat,
        }
    }
}

impl Actor for Audience {
    type Context = Context<Self>;
}


//  ----- Messages & Handlers -----

pub struct NewChatCommentReceived {
    pub comment: Comment,
}

impl Message for NewChatCommentReceived {
    type Result = ();
}

impl Handler<NewChatCommentReceived> for Audience {
    type Result = ();

    fn handle(&mut self, msg: NewChatCommentReceived, ctx: &mut Self::Context) -> Self::Result {
        match self.audience_tx.upgrade() {
            Some(recipient) => {
                recipient.do_send(
                    Reflect(Effect::NewChatCommentReceived(msg.comment))
                ).unwrap();
            }

            None => {
                ctx.stop();
            }
        }
    }
}

pub struct NewChatCommentPostedFromThisAudience {
    pub text: String,
}

impl Message for NewChatCommentPostedFromThisAudience {
    type Result = ();
}

impl Handler<NewChatCommentPostedFromThisAudience> for Audience {
    type Result = ();

    fn handle(&mut self, msg: NewChatCommentPostedFromThisAudience, _: &mut Self::Context) -> Self::Result {
        self.chat.do_send(
            crate::actors::chat::NewChatCommentPosted {
                audience_id: self.id.clone(),
                text: msg.text,
            }
        );
    }
}

pub struct Reflect(pub Effect);

pub enum Effect {
    NewChatCommentReceived(Comment)
}

impl Message for Reflect {
    type Result = ();
}