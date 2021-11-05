import ws, {Socket} from 'k6/ws'
import {check} from 'k6'
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
    const audienceId = __VU.toString().padStart(8, '0') + '-40fe-8708-0d3238b6f5lk'
    const url = `ws://${env.chatServerHost}/rooms/ws/${roomId}?audienceId=${audienceId}`

    try {
        ws.connect(url, (socket) => {
            socket.on('open', () => {
                socketOpenedCounter.add(1)

                socket.setInterval(() => {
                    checkSendMessage(roomId, audienceId, socket)
                }, env.perSecSendChatComment * 1000)
            })

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

const checkSendMessage = (eventId: string, userId: string, socket: Socket) => {
    try {
        postNewChatComment(eventId, userId, socket)

        check({}, {
            'chat comment sent': (_) => true
        })
    } catch (e) {
        check({}, {
            'chat comment sent': (_) => false
        })
    }
}

// ----- send message
const postNewChatComment = (eventId: string, userId: string, socket: Socket) => {
    const comment: PostNewChatComment = {
        messageType: 'POST_NEW_CHAT_COMMENT',
        text: `text:user-${userId}`
    }

    socket.send(JSON.stringify(comment))
}

type PostNewChatComment = {
    messageType: 'POST_NEW_CHAT_COMMENT',
    text: string,
}

// ----- env
const chatServerHost =
    __ENV.CHAT_SERVER_HOST
        ? __ENV.CHAT_SERVER_HOST
        : 'localhost:3000'

const perSecSendChatComment =
    __ENV.PER_SEC_SEND_CHAT_COMMENT
        ? Number(__ENV.PER_SEC_SEND_CHAT_COMMENT)
        : 5

const durationSeconds =
    __ENV.DURATION_SECONDS
        ? Number(__ENV.DURATION_SECONDS)
        : 30

const nUser =
    __ENV.N_USER
        ? Number(__ENV.N_USER)
        : 10

const env = {
    chatServerHost,
    perSecSendChatComment,
    durationSeconds,
    nUser
}

export let options: Options = {
    scenarios: {
        basic: {
            executor: 'ramping-vus',
            stages: [
                {duration: `${env.durationSeconds}s`, target: env.nUser}
            ],
            startVUs: env.nUser,
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
