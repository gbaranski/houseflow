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

// JWTAccessSecretEnv ust be set in .env
const JWTAccessSecretEnv = "AUTH_JWT_ACCESS_KEY"

// JWTRefreshSecretEnv  must be set in .env
const JWTRefreshSecretEnv = "AUTH_JWT_REFRESH_KEY"

// AccessToken is access token, this is held in redis
type AccessToken struct {
	Token   string
	UUID    uuid.UUID
	Expires int64
}

// AccessTokenMetadata is metadata for access token, this comes from JWT stringified token
type AccessTokenMetadata struct {
	UUID    uuid.UUID
	UserID  string
	Expires int64
}

// RefreshToken is access token, this is held in redis
type RefreshToken struct {
	Token   string
	UUID    uuid.UUID
	Expires int64
}

// TokenDetails describes details of token
type TokenDetails struct {
	RefreshToken *RefreshToken
	AccessToken  *AccessToken
}

func createAccessToken(userID primitive.ObjectID) (*AccessToken, error) {
	jwtKey := os.Getenv(JWTAccessSecretEnv)

	expires := time.Now().Add(time.Minute * 15).Unix()
	UUID, err := uuid.NewRandom()
	if err != nil {
		return nil, err
	}
	claims := jwt.MapClaims{
		"uuid":    UUID,
		"user_id": userID.Hex(),
		"exp":     expires,
	}
	token, err := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString([]byte(jwtKey))

	if err != nil {
		return nil, err
	}
	return &AccessToken{
		Token:   token,
		UUID:    UUID,
		Expires: expires,
	}, nil
}

func createRefreshToken(userID primitive.ObjectID) (*RefreshToken, error) {
	expires := time.Now().Add(time.Hour * 24 * 7).Unix()

	jwtKey := os.Getenv(JWTRefreshSecretEnv)
	UUID, err := uuid.NewRandom()
	if err != nil {
		return nil, err
	}
	claims := jwt.MapClaims{
		"uuid":    UUID.String(),
		"user_id": userID.Hex(),
		"exp":     expires,
	}
	token, err := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString([]byte(jwtKey))
	if err != nil {
		return nil, err
	}
	return &RefreshToken{
		Token:   token,
		UUID:    UUID,
		Expires: expires,
	}, nil

}

// CreateToken creates token details
func CreateToken(userID primitive.ObjectID) (*TokenDetails, error) {
	accessToken, err := createAccessToken(userID)
	if err != nil {
		return nil, err
	}
	refreshToken, err := createRefreshToken(userID)
	if err != nil {
		return nil, err
	}

	return &TokenDetails{
		RefreshToken: refreshToken,
		AccessToken:  accessToken,
	}, nil
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

// VerifyToken verifyes jwt token
func VerifyToken(strtoken string) (*jwt.Token, error) {
	token, err := jwt.Parse(strtoken, func(token *jwt.Token) (interface{}, error) {
		//Make sure that the token method conform to "SigningMethodHMAC"
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return []byte(os.Getenv(JWTAccessSecretEnv)), nil
	})
	if err != nil {
		return nil, err
	}
	if _, ok := token.Claims.(jwt.Claims); !ok && !token.Valid {
		return nil, err
	}

	return token, nil
}

// ExtractTokenMetadata extracts token metadata
func ExtractTokenMetadata(token jwt.Token) (*AccessTokenMetadata, error) {
	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return nil, fmt.Errorf("Unable to extract token metadata")
	}

	UUIDStr, ok := claims["uuid"].(string)
	if !ok {
		return nil, fmt.Errorf("Unable to retreive access_uuid from claims")
	}
	UUID, err := uuid.Parse(UUIDStr)
	if err != nil {
		return nil, err
	}

	userID, ok := claims["user_id"].(string)

	if !ok {
		return nil, fmt.Errorf("Unable to retreive user_id from claims")
	}

	expires, ok := claims["exp"].(int64)
	if !ok {
		return nil, fmt.Errorf("Unable to retreive exp from claims")
	}

	return &AccessTokenMetadata{
		UUID:    UUID,
		UserID:  userID,
		Expires: expires,
	}, nil
}
