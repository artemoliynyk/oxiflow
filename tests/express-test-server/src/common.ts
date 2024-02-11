import { Request, Response, NextFunction } from 'express'


export function calculateDelay(delay1: number, delay2: number | undefined): number {

    if (delay2) {
        const rangeFrom = Math.min(delay1, delay2);
        const rangeTo = Math.max(delay1, delay2);

        return Math.round(Math.random() * (rangeTo - rangeFrom) + rangeFrom);
    }

    return delay1;
}

export function checkTerminationParam(req: Request, res: Response, next: NextFunction) {
    // if parameter provided – terminate session sometimes "randomly"
    if ('terminate' in req.query && Math.random() > 0.8) {
        console.log("Terminating session");
        res.socket?.destroy();
        return;
    }

    // no termination, proceed with routes
    next()
}