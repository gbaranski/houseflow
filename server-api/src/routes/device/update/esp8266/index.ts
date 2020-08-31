import express from 'express';

const router = express.Router();


router.use((req, res, next) => {
    if (!req.get('x-esp8266-sta-mac') || !req.get('x-esp8266-version')) {
        throw new Error("Invalid request for ESP8266");
    }
    next();
})

router.all('/watermixer', (req, res) => {
    // const version = req.get('x-esp8266-version');
    // const mac = req.get('x-esp8266-sta-mac');

    res.sendStatus(304);
})

export default router;
