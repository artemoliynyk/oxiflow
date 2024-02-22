import Router, { Request, Response } from 'express'
import { calculateDelay, randomCode } from '../common'

const router = Router()

// Return requested code
router.all('/:code([1-5]{1}[\\d]{2})', async (req: Request, res: Response) => {
    const code = parseInt(req.params['code'])
    res.status(code).send("Response");
});

export default router;