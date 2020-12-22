import { getFirebaseUserByGoogleClientID } from '@/services/firebase';
import express from 'express';

const router = express.Router();

router.get(
  '/login',
  async (req, res): Promise<void> => {
    const params = {
      // The Google client ID you registered with Google.
      clientID: req.query['client_id'],
      // The URL to which you send the response to this request.
      redirectURI: req.query['redirect_uri'],
      // A bookkeeping value that is passed back to Google unchanged in the redirect URI.
      state: req.query['state'],
      // Optional: A space-delimited set of scope strings that specify the data Google is requesting authorization for.
      scope: req.query['scope'],
      // The type of value to return in the response. For the OAuth 2.0 authorization code flow, the response type is always code.
      responseType: req.query['response_type'],
      // The Google Account language setting in RFC5646 format, used to localize your content in the user's preferred language.
      userLocale: req.query['user_locale'],
    };
    console.log(params);
    if (typeof params.clientID !== 'string') throw new Error('Wrong ClientID');
    const users = await getFirebaseUserByGoogleClientID(params.clientID);
    console.log({ notFound: users.notFound });
    console.log({ users: users.users });

    res.status(400).send('Hi');
  },
);

router.post('/token', (req, res) => {
  console.log({ body: req.body });
  res.status(400).send('Hello world');
});

export default router;
