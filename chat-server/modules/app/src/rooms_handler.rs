use actix::{Actor, ActorContext, Addr, Handler, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};

use domain::actors::audience::{Audience, Effect, Reflect};
use domain::actors::entrance::{AudienceArrived, Entrance};

#[actix_web::get("/rooms/ws/{room_id}")]
pub async fn room_ws_index(
    request: HttpRequest,
    path_var: web::Path<RoomPathVar>,
    query: web::Query<RoomQuery>,
    stream: web::Payload,
    entrance: web::Data<Addr<Entrance>>,
) -> Result<HttpResponse, Error> {
    let room_id = path_var.room_id.clone();
    let audience_id = query.0.audience_id;

    println!(">>>>> Try to join /rooms/ws/{:?} with user {:?}", room_id, audience_id);
    let (addr, res) = ws::start_with_addr(
        RoomsWsHandler::new(),
        &request,
        stream,
    )?;

    // TODO: 力づく感がいなめないので直したい
    bind_audience(addr, room_id, audience_id, entrance.get_ref().clone()).await;

    Ok(res)
}

#[derive(serde::Deserialize)]
pub struct RoomPathVar {
    pub room_id: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomQuery {
    pub audience_id: String,
}


struct RoomsWsHandler {
    audience: Option<Addr<Audience>>,
}

impl RoomsWsHandler {
    fn new() -> Self {
        RoomsWsHandler {
            audience: None,
        }
    }

    fn audience(&mut self, audience: Addr<Audience>) {
        self.audience = Some(audience);
    }

    fn audience_addr(&self) -> Addr<Audience> {
        self.audience.as_ref().unwrap().clone()
    }
}

async fn bind_audience(
    addr: Addr<RoomsWsHandler>,
    room_id: String,
    audience_id: String,
    entrance: Addr<Entrance>,
) {
    let room =
        entrance.send(
            AudienceArrived {
                room_id
            }
        ).await.unwrap();

    let audience_tx = addr.downgrade().recipient();
    let audience =
        room.send(
            domain::actors::room::AudienceJoined {
                audience_id,
                audience_tx,
            }
        ).await.unwrap();

    // NOTE: mailbox の queue に積まれて upgrade 後に処理されるぽい（？）
    addr.do_send(AudienceJoinedToRoom { audience });
}

impl Actor for RoomsWsHandler {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!(">>>>> RoomsWsHandler started");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!(">>>>> RoomsWsHandler stopped");
    }
}

struct AudienceJoinedToRoom {
    audience: Addr<Audience>,
}

impl actix::Message for AudienceJoinedToRoom {
    type Result = ();
}

impl Handler<AudienceJoinedToRoom> for RoomsWsHandler {
    type Result = ();

    fn handle(&mut self, msg: AudienceJoinedToRoom, _: &mut Self::Context) -> Self::Result {
        self.audience(msg.audience);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RoomsWsHandler {
    fn handle(&mut self, message: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match message {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text.to_string()) {
                    Ok(client_message) => {
                        match client_message {
                            ClientMessage::PostNewChatComment { text } => {
                                self.audience_addr().do_send(
                                    domain::actors::audience::NewChatCommentPostedFromThisAudience {
                                        text
                                    }
                                );
                            }
                        }
                    }

                    Err(_) => {
                        println!(">>>>> Unable to parse text: {:?}", text);
                    }
                };
            }

            Ok(ws::Message::Ping(b)) => {
                ctx.pong(&b);
            }

            Ok(ws::Message::Close(_)) | Err(_) => {
                ctx.stop();
            }

            msg => {
                println!(">>>>> Unhandled message received: {:?}", msg);
            }
        }
    }
}

impl Handler<domain::actors::audience::Reflect> for RoomsWsHandler {
    type Result = ();

    fn handle(&mut self, msg: Reflect, ctx: &mut Self::Context) -> Self::Result {
        match msg.0 {
            Effect::NewChatCommentReceived(comment) => {
                ctx.text(
                    serde_json::to_string(
                        &ServerMessage::NewChatCommentReceived {
                            comment_id: comment.id,
                            room_id: comment.room_id,
                            audience_id: comment.audience_id,
                            text: comment.text,
                        }
                    ).unwrap()
                );
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "messageType", rename_all = "SCREAMING_SNAKE_CASE")]
enum ClientMessage {
    #[serde(rename_all = "camelCase")]
    PostNewChatComment {
        text: String
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "messageType", rename_all = "SCREAMING_SNAKE_CASE")]
enum ServerMessage {
    #[serde(rename_all = "camelCase")]
    NewChatCommentReceived {
        comment_id: String,
        room_id: String,
        audience_id: String,
        text: String,
    }
}
