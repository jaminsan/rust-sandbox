```shell
$ yarn run bundle --watch

# run receiver
$ k6 run -e N_USER=100 -e REACH_N_USER_DURATION_SECONDS=5 -e RECEPTION_DURATION_SECONDS=60 -e CHAT_SERVER_HOST=localhost:3000 dist/receiver.bundle.js

# run sender
$ k6 run -e N_USER=10 -e PER_SEC_SEND_CHAT_COMMENT=5 -e DURATION_SECONDS=15 -e CHAT_SERVER_HOST=localhost:3000 dist/sender.bundle.js
```

with kong, specify CHAT_SERVER_HOST env variables to localhost:80/chat