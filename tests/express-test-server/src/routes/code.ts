import Router, { Request, Response } from 'express'
import { calculateDelay } from '../common'

const router = Router()

function randomCode(): number {
    const codeHi = Math.round(Math.random() * (5 - 1) + 1);
    const codeLo = Math.round(Math.random() * 4);

    return codeHi * 100 + codeLo;
}

// Return requested code
router.all('/:code([1-5]{1}[\\d]{2})', async (req: Request, res: Response) => {
    let code = parseInt(req.params['code'])

    res.status(code).send("Response");
});

// generate random HTTP response code within the range 100-504
router.all('/random', async (req: Request, res: Response) => {
    const code = randomCode();
    res.status(code).send(`Response code ${code}`);
});

// generate random code response with the defined delay
const delayRoutePt1 = '/random/delay/:delay1([1-9]+[\\d]{0,})';
router.all(delayRoutePt1, async (req: Request, res: Response) => {
    const delay = parseInt(req.params['delay1']);
    await new Promise(resolve => setTimeout(resolve, delay)); // sleep


    const code = randomCode();
    res.status(code).send(`Delay ${delay}, Response code ${code}`);
});

// generate random code response with the random delay within the defined range
router.all(`${delayRoutePt1}-:delay2([1-9]+[\\d]+)`, async (req: Request, res: Response) => {
    let delay = calculateDelay(parseInt(req.params['delay1']), parseInt(req.params['delay2']));

    await new Promise(resolve => setTimeout(resolve, delay));
    const code = randomCode();
    res.status(code).send(`Delay ${delay}, Response code ${code}`);
});

export default router;