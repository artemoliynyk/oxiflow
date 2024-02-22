import express, { Request, Response } from 'express'
import RouterDelay from './routes/delay'
import RouterRandom from './routes/random'
import RouterCode from './routes/code'
import { checkTerminationParam } from './common'

const app = express();

app.use(checkTerminationParam);

app.get('/', (req, res) => res.send("It works!"));

app.use('/delay', RouterDelay);
app.use('/random', RouterRandom);
app.use('/code', RouterCode);

app.listen(8083, () => "Express Test Server has started");