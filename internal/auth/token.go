package auth

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
)

func (a *Auth) onTokenAuthorizationCodeGrant(w http.ResponseWriter, r *http.Request, form TokenQuery) {
	if !a.validateRedirectURI(form.RedirectURI) {
		json, _ := json.Marshal(types.ResponseError{
			Name: "invalid_redirect_uri",
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	token, err := utils.VerifyToken(form.Code, []byte(a.opts.AuthorizationCodeKey))
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "invalid_grant",
			Description: fmt.Sprintf("authorization code %s", err.Error()),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	_, rtstr, err := a.newRefreshToken(token.Audience)
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "rt_create_fail",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(json)
		return
	}

	_, atstr, err := a.newAccessToken(token.Audience)
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "at_create_fail",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(json)
		return
	}

	json, _ := json.Marshal(AuthorizationCodeGrantResponse{
		TokenType:    "Bearer",
		AccessToken:  atstr,
		RefreshToken: rtstr,
		ExpiresIn:    int(utils.AccessTokenDuration.Seconds()),
	})
	w.Write(json)
}

func (a *Auth) onRefreshTokenGrant(w http.ResponseWriter, r *http.Request, form TokenQuery) {
	rt, err := utils.VerifyToken(form.RefreshToken, []byte(a.opts.RefreshKey))
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "invalid_grant",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	_, atstr, err := a.newAccessToken(rt.Audience)
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "fail_create_at",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(json)
		return
	}

	json, _ := json.Marshal(RefreshTokenGrantResponse{
		TokenType:   "Bearer",
		AccessToken: atstr,
		ExpiresIn:   int(utils.AccessTokenDuration.Seconds()),
	})
	w.Write(json)
}

func (a *Auth) onToken(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	err := r.ParseForm()
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "fail_parse_form",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write(json)
		return
	}

	var form TokenQuery
	if err = decoder.Decode(&form, r.PostForm); err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "fail_parse_form",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write(json)
		return
	}

	if form.ClientID != a.opts.ClientID || form.ClientSecret != a.opts.ClientSecret {
		json, _ := json.Marshal(types.ResponseError{
			Name: "invalid_oauth_credentials",
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	if form.GrantType == "authorization_code" {
		a.onTokenAuthorizationCodeGrant(w, r, form)
	} else if form.GrantType == "refresh_token" {
		a.onRefreshTokenGrant(w, r, form)
	} else {
		json, _ := json.Marshal(types.ResponseError{
			Name: "unknown_grant_type",
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
	}
}
