package utils

import (
	"fmt"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/dgrijalva/jwt-go"
	"github.com/google/uuid"
)

// JWTAccessSecretEnv ust be set in .env
const JWTAccessSecretEnv = "AUTH_JWT_ACCESS_KEY"

// JWTRefreshSecretEnv  must be set in .env
const JWTRefreshSecretEnv = "AUTH_JWT_REFRESH_KEY"

// TokenClaims is claims for jwt token
type TokenClaims struct {
	jwt.StandardClaims
}

// TokenDetails combines token string and claims
type TokenDetails struct {
	Token  string
	Claims *TokenClaims
}

// Tokens combins both access token and refresh token
type Tokens struct {
	AccessToken  TokenDetails
	RefreshToken TokenDetails
}

func createJWTToken(expiresAt time.Time, secretEnv string) (*TokenClaims, *string, error) {
	jwtKey := os.Getenv(secretEnv)

	id, err := uuid.NewRandom()
	if err != nil {
		return nil, nil, err
	}
	claims := &TokenClaims{
		jwt.StandardClaims{
			ExpiresAt: expiresAt.Unix(),
			Id:        id.String(),
		},
	}
	token, err := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString([]byte(jwtKey))

	if err != nil {
		return nil, nil, err
	}
	return claims, &token, nil
}

// CreateTokens creates tokens
func CreateTokens() (*Tokens, error) {
	accessTokenDuration := time.Now().Add(time.Minute * 15)
	accessTokenClaims, accessToken, err := createJWTToken(accessTokenDuration, JWTAccessSecretEnv)
	if err != nil {
		return nil, err
	}
	refreshTokenDuration := time.Now().Add(time.Hour * 24 * 7)
	refreshTokenClaims, refreshToken, err := createJWTToken(refreshTokenDuration, JWTRefreshSecretEnv)
	if err != nil {
		return nil, err
	}

	return &Tokens{
		RefreshToken: TokenDetails{
			Token:  *refreshToken,
			Claims: refreshTokenClaims,
		},
		AccessToken: TokenDetails{
			Token:  *accessToken,
			Claims: accessTokenClaims,
		},
	}, nil
}

// VerifyToken verifyes jwt token, secretEnv must be some enviroent variable
func VerifyToken(strtoken string, secretEnv string) (*jwt.Token, *TokenClaims, error) {
	token, err := jwt.ParseWithClaims(strtoken, &TokenClaims{}, func(token *jwt.Token) (interface{}, error) {
		//Make sure that the token method conform to "SigningMethodHMAC"
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return []byte(os.Getenv(secretEnv)), nil
	})
	if err != nil {
		return nil, nil, err
	}
	claims, ok := token.Claims.(*TokenClaims)
	if !ok || !token.Valid {
		return nil, nil, fmt.Errorf("Failed parsing claims")
	}
	return token, claims, nil
}

// ExtractToken extracts token from Bearer
func ExtractToken(r *http.Request) *string {
	bearToken := r.Header.Get("Authorization")
	//normally Authorization the_token_xxx
	strArr := strings.Split(bearToken, " ")
	if len(strArr) == 2 {
		return &strArr[1]
	}
	return nil
}
