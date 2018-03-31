import { Client, Message, Collection, Member } from 'eris';
import Parachute = require('./Parachute');

function auto_grouping(client: Client, message: Message) {
  const members: Member[] = (() => {
    const members: Member[] = [];
    // 空ならどうしようもない
    if (message.member && message.member.voiceState.channelID) {
      try {
        const channel:any = client.getChannel(message.member.voiceState.channelID);
        if (channel.voiceMembers) {
          channel.voiceMembers.forEach((member: Member) => {	
            members.push(member);
          });                
        }						
      } catch (e) {
        console.error(e);
      }
    }
    return members;        
  })();

  let mes: string = '';

  // 人数が足りない場合
  if (members.length < 4) {
    mes = '人数が足りてないんじゃない？';
  } else {
  	// シャッフルを行う
  	shuffle_members(members);
        
  	// メッセージの組み立て
  	mes += 'TEAM1:';
    for (let i = 0; i < members.length; ++i) {
      if (i === Math.floor(members.length / 2)) {
    	   mes += '\nTEAM2:';
      }
      mes += ` ${members[i].username}`;
    }
  }

  // メッセージの送信
  try {
    message.channel.createMessage(mes);
  } catch (e) {
    console.log(e);
  }
}

function shuffle_members(members: Member[]) {
  // Fisher-Yates shuffle
  for (let i = members.length - 1; i > 0; i--) {
    const n = Math.floor(Math.random() * (i + 1));
    const tmp = members[i];
    members[i] = members[n];
    members[n] = tmp;
  }
}

export = { label: 'team', command: auto_grouping, permission: Parachute.Permission.USER };
