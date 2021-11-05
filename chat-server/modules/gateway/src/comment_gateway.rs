use actix::{Actor, Context, Handler, Supervised, SystemService};
use actix_broker::BrokerSubscribe;
use futures::StreamExt;

use domain::actors::chat::{ChatCommentSaved, SaveChatComment, SubscribeChatComment};
use domain::chat::comment::Comment;
use driver::comment_driver;
use driver::comment_driver::CommentValue;
use port::comment_port::CommentPort;

pub struct CommentGateway;

impl Default for CommentGateway {
    fn default() -> Self {
        CommentGateway
    }
}

impl CommentPort<CommentGateway> for CommentGateway {}

impl Actor for CommentGateway {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<SaveChatComment>(ctx);
        self.subscribe_system_async::<SubscribeChatComment>(ctx);
    }
}

impl Handler<SaveChatComment> for CommentGateway {
    type Result = ();

    fn handle(&mut self, msg: SaveChatComment, _: &mut Self::Context) -> Self::Result {
        tokio::spawn(async move {
            let comment_value = convert_to_comment_value(msg.comment);

            comment_driver::publish(comment_value.clone()).await;
            comment_driver::push(comment_value).await;
        });
    }
}

impl Handler<SubscribeChatComment> for CommentGateway {
    type Result = ();

    fn handle(&mut self, msg: SubscribeChatComment, _: &mut Self::Context) -> Self::Result {
        tokio::spawn(async move {
            let mut stream = comment_driver::subscribe(&msg.room_id).await;
            let addr = msg.chat.clone();
            while let Some(comment_value) = stream.next().await {
                addr.send(
                    ChatCommentSaved { comment: convert_to_comment(comment_value) }
                ).await.unwrap();
            }
        });
    }
}

impl SystemService for CommentGateway {}

impl Supervised for CommentGateway {}

fn convert_to_comment(cv: CommentValue) -> Comment {
    Comment {
        id: cv.comment_id,
        room_id: cv.room_id,
        audience_id: cv.audience_id,
        text: cv.text,
    }
}

fn convert_to_comment_value(c: Comment) -> CommentValue {
    CommentValue {
        comment_id: c.id,
        room_id: c.room_id,
        audience_id: c.audience_id,
        text: c.text,
    }
}