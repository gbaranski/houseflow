package auth

import (
	"fmt"
	"math"
	"net/url"
	"time"

	"github.com/gbaranski/houseflow/pkg/token"
)

func (a Auth) validateRedirectURI(uri string) bool {
	return uri == fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", a.opts.ProjectID) || uri == fmt.Sprintf("https://oauth-redirect-sandbox.googleusercontent.com/r/%s", a.opts.ProjectID)
}

func (a Auth) newRefreshToken(aud []byte) (token.Signed, error) {
	token := token.Parsed{
		Audience:  aud,
		ExpiresAt: math.MaxUint32,
	}
	return token.Sign([]byte(a.opts.RefreshKey))
}

func (a Auth) newAccessToken(aud []byte) (token.Signed, error) {
	token := token.Parsed{
		Audience:  aud,
		ExpiresAt: uint32(time.Now().Add(token.AccessTokenDuration).Unix()),
	}
	return token.Sign([]byte(a.opts.AccessKey))
}

func (a *Auth) createRedirectURI(q LoginPageQuery, aud []byte) (string, error) {
	token := token.Parsed{
		Audience:  aud,
		ExpiresAt: uint32(time.Now().Add(token.AuthorizationCodeDuration).Unix()),
	}
	strtoken, err := token.Sign([]byte(a.opts.AuthorizationCodeKey))
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("%s?code=%s&state=%s", q.RedirectURI, url.QueryEscape(string(strtoken.Base64())), q.State), nil
}
