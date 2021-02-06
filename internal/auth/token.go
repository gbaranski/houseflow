package auth

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gbaranski/houseflow/pkg/token"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/sirupsen/logrus"
)

func (a *Auth) onTokenAuthorizationCodeGrant(w http.ResponseWriter, r *http.Request, form TokenQuery) {
	if !a.validateRedirectURI(form.RedirectURI) {
		logrus.Errorf("Invalid redirect URI\n")
		utils.ReturnError(w, types.ResponseError{
			Name:       "invalid_redirect_uri",
			StatusCode: 400,
		})
		return
	}

	signedCode, err := token.NewSignedFromBase64([]byte(form.Code))
	if err != nil {
		logrus.Errorf("Malformed authorization code %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "invalid_grant",
			Description: fmt.Sprintf("invalid authorization code %s", err.Error()),
			StatusCode:  400,
		})
		return
	}
	if err = signedCode.Verify([]byte(a.opts.AuthorizationCodeKey)); err != nil {
		logrus.Errorf("Invalid authorization code %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "invalid_grant",
			Description: fmt.Sprintf("invalid authorization code %s", err.Error()),
			StatusCode:  400,
		})
		return
	}
	parsed := signedCode.Parse()

	rt, err := a.newRefreshToken(parsed.Audience)
	if err != nil {
		logrus.Errorf("Fail creating Refresh Token %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "rt_create_fail",
			Description: err.Error(),
			StatusCode:  500,
		})
		return
	}

	at, err := a.newAccessToken(parsed.Audience)
	if err != nil {
		logrus.Errorf("Fail creating Access Token %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "at_create_fail",
			Description: err.Error(),
			StatusCode:  500,
		})
		return
	}

	logrus.Infof("Granted Authorization Code for User ID: %s\n", signedCode.Parse().Audience)
	json, _ := json.Marshal(AuthorizationCodeGrantResponse{
		TokenType:    "Bearer",
		AccessToken:  string(at.Base64()),
		RefreshToken: string(rt.Base64()),
		ExpiresIn:    int(token.AccessTokenDuration.Seconds()),
	})
	w.Write(json)
}

func (a *Auth) onRefreshTokenGrant(w http.ResponseWriter, r *http.Request, form TokenQuery) {
	signedRT, err := token.NewSignedFromBase64WithVerify([]byte(a.opts.RefreshKey), []byte(form.RefreshToken))
	if err != nil {
		logrus.Errorf("Failed verifying signed token: %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "invalid_grant",
			Description: err.Error(),
			StatusCode:  http.StatusBadRequest,
		})
		return
	}

	signedAT, err := a.newAccessToken(signedRT.Parse().Audience)
	if err != nil {
		logrus.Errorf("Failed creating AccessToken: %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "fail_create_at",
			Description: err.Error(),
			StatusCode:  http.StatusInternalServerError,
		})
		return
	}

	logrus.Infof("Refreshed Access Token for User ID: %s\n", signedRT.Parse().Audience)
	json, _ := json.Marshal(RefreshTokenGrantResponse{
		TokenType:   "Bearer",
		AccessToken: string(signedAT.Base64()),
		ExpiresIn:   int(token.AccessTokenDuration.Seconds()),
	})
	w.Write(json)
}

func (a *Auth) onToken(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	err := r.ParseForm()
	if err != nil {
		logrus.Errorf("Fail parsing form: %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:       "fail_parse_form",
			StatusCode: http.StatusUnprocessableEntity,
		})
		return
	}

	var form TokenQuery
	if err = decoder.Decode(&form, r.PostForm); err != nil {
		logrus.Errorf("Fail decoding PostForm: %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:       "fail_decode_form",
			StatusCode: http.StatusUnprocessableEntity,
		})
		return
	}

	if form.ClientID != a.opts.ClientID || form.ClientSecret != a.opts.ClientSecret {
		logrus.Errorf("Invalid OAuth2 credentials, ClientID: %s, ClientSecret: %s\n", form.ClientID, form.ClientSecret)
		utils.ReturnError(w, types.ResponseError{
			Name:       "invalid_oauth_credentials",
			StatusCode: http.StatusBadRequest,
		})
		return
	}

	if form.GrantType == "authorization_code" {
		logrus.Infoln("Authorization code grant requested")
		a.onTokenAuthorizationCodeGrant(w, r, form)
	} else if form.GrantType == "refresh_token" {
		logrus.Infoln("Refresh token grant requested")
		a.onRefreshTokenGrant(w, r, form)
	} else {
		logrus.Errorf("Unknown grant type: %s\n", form.GrantType)
		utils.ReturnError(w, types.ResponseError{
			Name:       "unknown_grant_type",
			StatusCode: http.StatusBadRequest,
		})
	}
}
