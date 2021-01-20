package auth

import (
	"fmt"
	"net/url"
	"time"

	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/google/uuid"
)

func (a Auth) validateRedirectURI(uri string) bool {
	return uri == fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", a.opts.ProjectID) || uri == fmt.Sprintf("https://oauth-redirect-sandbox.googleusercontent.com/r/%s", a.opts.ProjectID)
}

func (a Auth) newRefreshToken(userID string) (token utils.Token, strtoken string, err error) {
	id, err := uuid.NewRandom()
	if err != nil {
		return
	}
	token = utils.Token{
		Audience: userID,
		ID:       id.String(),
	}
	strtoken, err = token.Sign([]byte(a.opts.RefreshKey))
	return
}

func (a Auth) newAccessToken(userID string) (token utils.Token, strtoken string, err error) {
	id, err := uuid.NewRandom()
	if err != nil {
		return
	}
	token = utils.Token{
		Audience:  userID,
		ID:        id.String(),
		ExpiresAt: time.Now().Add(utils.AccessTokenDuration).Unix(),
	}
	strtoken, err = token.Sign([]byte(a.opts.AccessKey))
	return
}

func (a *Auth) createRedirectURI(q LoginPageQuery, userID string) (string, error) {
	token := utils.Token{
		Audience:  userID,
		ExpiresAt: time.Now().Add(utils.AuthorizationCodeDuration).Unix(),
	}
	strtoken, err := token.Sign([]byte(a.opts.AuthorizationCodeKey))
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("%s?code=%s&state=%s", q.RedirectURI, url.QueryEscape(strtoken), q.State), nil
}
