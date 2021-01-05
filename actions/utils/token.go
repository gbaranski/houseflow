package utils

import (
	"fmt"
	"os"

	"github.com/dgrijalva/jwt-go"
)

// JWTAccessKey ust be set in .env
const JWTAccessKey = "JWT_ACCESS_KEY"

// JWTRefreshKey  must be set in .env
const JWTRefreshKey = "JWT_REFRESH_KEY"

// JWTAuthCodeKey  must be set in .env
const JWTAuthCodeKey = "JWT_AUTHORIZATION_CODE_KEY"

// TokenClaims is claims for jwt token
type TokenClaims struct {
	jwt.StandardClaims
}

// TokenDetails combines token string and claims
type TokenDetails struct {
	Token  *jwt.Token
	Claims TokenClaims
}

// VerifyToken verifyes jwt token, secretEnv must be some enviroent variable
func VerifyToken(strtoken string, secretEnv string) (*TokenDetails, error) {
	token, err := jwt.ParseWithClaims(strtoken, &TokenClaims{}, func(token *jwt.Token) (interface{}, error) {
		//Make sure that the token method conform to "SigningMethodHMAC"
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return []byte(os.Getenv(secretEnv)), nil
	})
	if err != nil {
		return nil, err
	}
	claims, ok := token.Claims.(*TokenClaims)
	if !ok || !token.Valid {
		return nil, fmt.Errorf("failed parsing claims")
	}
	return &TokenDetails{
		Token:  token,
		Claims: *claims,
	}, nil
}
