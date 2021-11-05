use actix::{Actor, Handler};

use domain::actors::chat::{SaveChatComment, SubscribeChatComment};

pub trait CommentPort<
    T: Actor
    + Handler<SaveChatComment>
    + Handler<SubscribeChatComment>
> {}
