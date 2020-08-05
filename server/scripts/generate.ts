import { v4 as uuidv4 } from 'uuid';

for (let i = 0; i < 5; i++) {
  const deviceUid = uuidv4();
  console.log(deviceUid);
}
