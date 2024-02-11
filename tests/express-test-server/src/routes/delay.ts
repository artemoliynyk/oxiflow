import Router, { Request, Response } from 'express'
import { calculateDelay, checkTerminationParam } from '../common'

const router = Router()

const delayUrlPart1 = '/:delay1([1-9]+[\\d]{0,})';

// send 200 reponse with delay within the range
router.all(delayUrlPart1, async (req: Request, res: Response) => {
    let delay = parseInt(req.params['delay1']);
    await new Promise(resolve => setTimeout(resolve, delay));

    res.send(`Response time ${delay} ms`);
});

// send 200 reponse with delay within the range
router.all(`${delayUrlPart1}-:delay2([1-9]+[\\d]+)`, async (req: Request, res: Response) => {
    let delay = calculateDelay(parseInt(req.params['delay1']), parseInt(req.params['delay2']));
    await new Promise(resolve => setTimeout(resolve, delay));

    res.send(`Response time ${delay} ms`);
});

export default router;