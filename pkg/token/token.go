package token

import (
	"net/http"
	"strings"
	"time"
)

const (
	// AuthorizationCodeDuration defines expire duration for Authorization Code
	AuthorizationCodeDuration = time.Minute * 10
	// AccessTokenDuration defines expire duration for Access Token
	AccessTokenDuration = time.Hour

	// TokenAudienceSize is fixed size of Audience in Token
	TokenAudienceSize = 36

	// UnixTimestampSize says how many bytes there are required to store single unix timestamp
	UnixTimestampSize = 32 / 8
)

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
