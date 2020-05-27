import express from 'express';
import Alarmclock from './routes/alarmclock';

const httpPort = 8080;

const app = express();
const alarmClock = new Alarmclock();
app.use(express.json()); // for parsing application/json

// app.post('/getAlarmClock', (req, res) => {
//   console.log(req.body);
//   res.json(req.body);
// });

setInterval(alarmClock.fetchEspDataInterval, 1000);

app.post('/getAlarmClock', (req, res) => alarmClock.handleRequest(req, res));

app.listen(httpPort, () => console.log(`Example app listening at http://localhost:${httpPort}`));
