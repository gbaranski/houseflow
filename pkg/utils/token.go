package utils

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"time"

	"crypto/hmac"
	"crypto/sha256"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

const (
	// AuthorizationCodeDuration defines expire duration for Authorization Code
	AuthorizationCodeDuration = time.Minute * 10
	// AccessTokenDuration defines expire duration for Access Token
	AccessTokenDuration = time.Hour
)

// Token is some custom implementation ala JWT
type Token struct {
	Audience  string `json:"aud,omitempty"`
	ExpiresAt int64  `json:"exp,omitempty"`
	ID        string `json:"id,omitempty"`
}

func (t *Token) Sign(key []byte) (string, error) {
	payload, err := json.Marshal(t)
	if err != nil {
		return "", err
	}
	mac := hmac.New(sha256.New, key)
	_, err = mac.Write(payload)
	if err != nil {
		return "", err
	}

	sig := base64.StdEncoding.EncodeToString(mac.Sum(nil))

	payloadEnc := base64.StdEncoding.EncodeToString(payload)

	token := strings.Join([]string{sig, payloadEnc}, ".")

	return token, nil
}

func (t *Token) Equal(t2 Token) bool {
	return t.Audience == t2.Audience && t.ExpiresAt == t2.ExpiresAt && t.ID == t2.ID
}

func VerifyToken(strtoken string, key []byte) (*Token, error) {
	tokenSplitted := strings.Split(strtoken, ".")
	if len(tokenSplitted) != 2 {
		return nil, fmt.Errorf("token doesn't contain signature or payload")
	}
	sig, err := base64.StdEncoding.DecodeString(tokenSplitted[0])
	if err != nil {
    return nil, fmt.Errorf("fail decode sig: %s", err.Error())
	}
	payload, err := base64.StdEncoding.DecodeString(tokenSplitted[1])
	if err != nil {
    return nil, fmt.Errorf("fail decode payload: %s", err.Error())
	}
	mac := hmac.New(sha256.New, key)
	_, err = mac.Write(payload)
	if err != nil {
		return nil, err
	}
	expectedMAC := mac.Sum(nil)
	if !hmac.Equal(sig, expectedMAC) {
		return nil, fmt.Errorf("token is invalid")
	}
	var token Token
	err = json.Unmarshal(payload, &token)
	if err != nil {
		return nil, err
	}
	if token.ExpiresAt < time.Now().Unix() {
		return nil, fmt.Errorf("token is expired")
	}
	return &token, nil
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

func ExtractWithVerifyUserToken(r *http.Request, key []byte) (*primitive.ObjectID, error) {
	strtoken := ExtractHeaderToken(r)
	if strtoken == nil {
		return nil, fmt.Errorf("token header is missing")
	}
	token, err := VerifyToken(*strtoken, key)
	if err != nil {
		return nil, err
	}
  userID, err := primitive.ObjectIDFromHex(token.Audience)
  if err != nil {
    return nil, err
  }

	return &userID, nil
}
