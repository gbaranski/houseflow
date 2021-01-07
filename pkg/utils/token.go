package utils

import (
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/dgrijalva/jwt-go"
	"github.com/google/uuid"
)

// TokenClaims is claims for jwt token
type TokenClaims struct {
	jwt.StandardClaims
}

// TokenDetails combines token string and claims
type TokenDetails struct {
	Token  *jwt.Token
	Claims TokenClaims
}

const (
	// AuthorizationCodeDuration defines expire duration for Authorization Code
	AuthorizationCodeDuration = time.Minute * 10
	// AccessTokenDuration defines expire duration for Access Token
	AccessTokenDuration = time.Hour
)

// TokenOptions define options for creating JWT Token
type TokenOptions struct {
	// Duration after token will expire
	ExpiresAt time.Duration

	// Secret key used to generate JWT, must be present
	Key string

	// Defines audience claim, used for adding user_id
	Audience string
}

// CreateJWTToken creates JWT Token with options
func CreateJWTToken(opts TokenOptions) (*TokenDetails, error) {
	id, err := uuid.NewRandom()
	if err != nil {
		return nil, err
	}
	claims := TokenClaims{
		jwt.StandardClaims{
			Id:       id.String(),
			Audience: opts.Audience,
		},
	}
	if opts.ExpiresAt != 0 {
		claims.ExpiresAt = time.Now().Add(opts.ExpiresAt).Unix()
	}
	token, err := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString(opts.Key)
	if err != nil {
		return nil, err
	}
	return VerifyToken(token, []byte(opts.Key))
}

// VerifyToken verifyes jwt token, secretEnv must be some enviroent variable
func VerifyToken(strtoken string, key []byte) (*TokenDetails, error) {
	token, err := jwt.Parse(strtoken, func(token *jwt.Token) (interface{}, error) {
		//Make sure that the token method conform to "SigningMethodHMAC"
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return key, nil
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

// ExtractHeaderToken extracts Bearer authorization token
func ExtractHeaderToken(r *http.Request) *string {
	bearToken := r.Header.Get("Authorization")
	//normally Authorization the_token_xxx
	strArr := strings.Split(bearToken, " ")
	if len(strArr) == 2 {
		return &strArr[1]
	}
	return nil
}
