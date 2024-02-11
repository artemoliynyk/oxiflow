import express, { Request, Response } from 'express'
import RouterCode from './routes/code'
import RouterDelay from './routes/delay'
import { checkTerminationParam } from './common'

const app = express();

app.use(checkTerminationParam);

app.get('/', (req, res) => res.send("It works!"));

app.use('/code', RouterCode);
app.use('/delay', RouterDelay);

app.listen(8083, () => "Express Test Server has started");