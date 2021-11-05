import ws from 'k6/ws'
import {check, fail} from 'k6'
import {Options} from 'k6/options'
import {Counter} from 'k6/metrics'

export const setup = () => {
    console.log('ENV: ', JSON.stringify(env))
}

const socketOpenedCounter = new Counter('SocketOpened')
const socketClosedCounter = new Counter('SocketClosed')
const socketErrorCounter = new Counter('SocketError')
const badHandshakeCounter = new Counter('BadHandshake')

export default () => {
    const roomId = 'roomA'
    const audienceId = __VU.toString().padStart(8, '0')
    const url = `ws://${env.chatServerHost}/rooms/ws/${roomId}?audienceId=${audienceId}`

    try {
        ws.connect(url, (socket) => {

            socket.on('open', () => {
                socketOpenedCounter.add(1)
            })

            socket.on('message', checkReceiveMessage)

            socket.on('close', () => {
                console.log(`VU ${__VU}: socket disconnected`)
                socketClosedCounter.add(1)
            })

            socket.on('error', (e) => {
                console.log(`VU ${__VU}: socket error `, e)
                socketErrorCounter.add(1)
            })
        })
    } catch (e) {
        console.log(`VU ${__VU}: bad handshake `, e)
        badHandshakeCounter.add(1)
    }
}

// ----- receive message
const checkReceiveMessage = (message: string) => {
    const msg: ServerMessage = JSON.parse(message)

    switch (msg.messageType) {
        case 'NEW_CHAT_COMMENT_RECEIVED':
            check(msg, {
                'NEW_CHAT_COMMENT_RECEIVED message received': (m) => m.commentId.length > 0
            })
            break

        default:
            let m = JSON.stringify(msg)
            fail(`Unexpected comment type is received: ${m}`)
    }
}

type NewChatCommentReceived = {
    messageType: 'NEW_CHAT_COMMENT_RECEIVED',
    commentId: string,
    roomId: string,
    audienceId: string,
    text: string,
}

type ServerMessage =
    NewChatCommentReceived

// ----- env
const chatServerHost =
    __ENV.CHAT_SERVER_HOST
        ? __ENV.CHAT_SERVER_HOST
        : 'localhost:3331'

const receptionDurationSeconds =
    __ENV.RECEPTION_DURATION_SECONDS
        ? Number(__ENV.RECEPTION_DURATION_SECONDS)
        : 30

const nUser =
    __ENV.N_USER
        ? Number(__ENV.N_USER)
        : 10

const reachNUserDurationSeconds =
    __ENV.REACH_N_USER_DURATION_SECONDS
        ? Number(__ENV.REACH_N_USER_DURATION_SECONDS)
        : 5

const env = {
    chatServerHost,
    receptionDuration: receptionDurationSeconds,
    nUser,
    reachNUserDuration: reachNUserDurationSeconds
}

export let options: Options = {
    scenarios: {
        basic: {
            executor: 'ramping-vus',
            stages: [
                {duration: `${env.reachNUserDuration}s`, target: env.nUser},
                {duration: `${env.receptionDuration}s`, target: env.nUser}
            ],
            gracefulStop: '5s',
            gracefulRampDown: '5s'
        }
    },
    thresholds: {
        'BadHandshake': [`count<=0`],
        'SocketOpened': [`count=>${env.nUser}`],
        'SocketClosed': [`count=>${env.nUser}`],
        'SocketError': [`count<=0`]
    }
}