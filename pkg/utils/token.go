package utils

import (
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/dgrijalva/jwt-go"
	jose "github.com/dvsekhvalnov/jose2go"
	"github.com/google/uuid"
)

// TokenDetails combines token string and claims
type TokenDetails struct {
	Token  *jwt.Token
	Claims jwt.StandardClaims
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
	Duration time.Duration

	// Secret key used to generate JWT, must be present
	Key string

	// Defines audience claim, used for adding user_id
	Audience string
}

// CreateJWTToken creates JWT Token with options
func CreateJWTToken(opts TokenOptions) (*TokenDetails, error) {
	jose.Sign()
	id, err := uuid.NewRandom()
	if err != nil {
		return nil, err
	}
	claims := jwt.StandardClaims{
		Id:       id.String(),
		Audience: opts.Audience,
	}
	if opts.Duration != 0 {
		claims.ExpiresAt = time.Now().Add(opts.Duration).Unix()
	}
	if claims.Valid() != nil {
		return nil, fmt.Errorf("invalid claims: %s", claims.Valid().Error())
	}
	token, err := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString([]byte(opts.Key))
	if err != nil {
		return nil, err
	}
	return VerifyToken(token, []byte(opts.Key))
}

// VerifyToken verifyes jwt token, secretEnv must be some enviroent variable
func VerifyToken(strtoken string, key []byte) (*TokenDetails, error) {
	token, err := jwt.ParseWithClaims(strtoken, &jwt.StandardClaims{}, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return key, nil
	})
	if err != nil {
		return nil, err
	}
	if !token.Valid {
		return nil, fmt.Errorf("token is invalid")
	}

	if token.Claims.Valid() != nil {
		return nil, token.Claims.Valid()
	}

	claims, ok := token.Claims.(*jwt.StandardClaims)
	if claims.Valid() != nil {
		return nil, claims.Valid()
	}
	if !ok {
		return nil, fmt.Errorf("claims are invalid")
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
