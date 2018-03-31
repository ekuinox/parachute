import { Client, Message, Collection, Member } from 'eris';
import { Permission } from '../parachute';

function ping_pong(client: Client, message: Message) {
  try {
    message.channel.createMessage('pong!');
  } catch (e) {
    console.error(e);
  }
}

export = { label: 'ping', command: ping_pong, permission: Permission.USER };