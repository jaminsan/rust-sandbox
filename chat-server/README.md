run redis & kong proxy

```shell
$ cd environments
$ docker-compose up -d --build
```

run chat-server

```shell
$ RUST_LOG=debug cargo watch -x run
[2021-11-05T00:00:00Z INFO  actix_server::builder] Starting 8 workers
[2021-11-05T00:00:00Z INFO  actix_server::builder] Starting "actix-web-service-127.0.0.1:3000" service on TcpListener { addr: 127.0.0.1:3000, fd: 11 }
```

send chat comment

```shell
# websocket connection through proxy-kong
# proxy-kong proxies localhost:80/chat access to localhost:3000 
$ wscat -c "ws://localhost:80/chat/rooms/ws/roomC?audienceId=333"
Connected (press CTRL+C to quit)
> {"messageType":"POST_NEW_CHAT_COMMENT","text":"コメントです"}
< {"messageType":"NEW_CHAT_COMMENT_RECEIVED","commentId":"7271192d-523d-4eae-8e83-158ab4f1a5e5","roomId":"roomB","audienceId":"333","text":"コメントです"}
```