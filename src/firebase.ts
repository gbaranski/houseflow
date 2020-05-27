import * as admin from 'firebase-admin';

admin.initializeApp({
  credential: admin.credential.applicationDefault(),
});

export function sendMessage(username: string, requestTypeString: string) {
  const message = {
    name: 'Alert',
    data: {
      title: 'Home alert!',
      body: `${username} requested ${requestTypeString}!`,
    },
    notification: {
      title: 'Home alert!',
      body: `${username} requested ${requestTypeString}`,
    },
    topic: 'admin',
  };
  admin
    .messaging()
    .send(message)
    .then(response => {
      console.log('Successfully sent message:', response);
    })
    .catch(error => {
      console.log('Error sending message:', error);
    });
}
