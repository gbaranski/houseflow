import jwt from 'jsonwebtoken';

const { JWT_ACCESS_KEY } = process.env;
if (!JWT_ACCESS_KEY) throw new Error('JWT_ACCESS_KEY is not defined in .env');

export interface TokenClaims {
  jti: string;
  exp: number;
}

// Verifies token and returns claims of it, can throw error
export const verifyToken = (token: string): Promise<TokenClaims> => {
  return new Promise<TokenClaims>((resolve, reject) => {
    jwt.verify(
      token,
      JWT_ACCESS_KEY,
      {
        complete: true,
      },
      (err, decoded) => {
        if (err) {
          return reject(err);
        }
        if (!decoded) {
          return reject(new Error('Unable to decode token'));
        }
        // @ts-ignore
        const payload = decoded.payload as TokenClaims;
        if (!payload.jti)
          return reject(
            new Error('Unable to retreive `jti`  property on token'),
          );
        if (!payload.exp)
          return reject(
            new Error('Unable to retreive `exp`  property on token'),
          );
        resolve(payload);
      },
    );
  });
};
