package utils

import (
	"fmt"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/dgrijalva/jwt-go"
	"github.com/google/uuid"
	"go.mongodb.org/mongo-driver/bson/primitive"
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

func createJWTToken(expiresAt time.Time, secretEnv string, audience *string) (*TokenDetails, error) {
	jwtKey := os.Getenv(secretEnv)

	id, err := uuid.NewRandom()
	if err != nil {
		return nil, err
	}
	claims := TokenClaims{
		jwt.StandardClaims{
			Id: id.String(),
		},
	}
	if audience != nil {
		claims.Audience = *audience
	}
	if !expiresAt.IsZero() {
		claims.ExpiresAt = expiresAt.Unix()
	}
	strtoken, err := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString([]byte(jwtKey))
	if err != nil {
		return nil, err
	}

	return VerifyToken(strtoken, secretEnv)
}

// CreateAuthorizationCode creates authorization code
func CreateAuthorizationCode(userID primitive.ObjectID) (*TokenDetails, error) {
	userIDHex := userID.Hex()
	return createJWTToken(time.Now().Add(time.Minute*10), JWTAuthCodeKey, &userIDHex)
}

// CreateAccessToken creates acces token and returns it
func CreateAccessToken(userID primitive.ObjectID) (*TokenDetails, error) {
	userIDHex := userID.Hex()
	return createJWTToken(time.Now().Add(time.Hour), JWTAccessKey, &userIDHex)
}

// CreateRefreshToken creates acces token and returns it
func CreateRefreshToken(userID primitive.ObjectID) (*TokenDetails, error) {
	userIDHex := userID.Hex()
	return createJWTToken(time.Time{}, JWTRefreshKey, &userIDHex)
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

// ExtractToken extracts Bearer authorization token
func ExtractToken(r *http.Request) *string {
	bearToken := r.Header.Get("Authorization")
	//normally Authorization the_token_xxx
	strArr := strings.Split(bearToken, " ")
	if len(strArr) == 2 {
		return &strArr[1]
	}
	return nil
}
