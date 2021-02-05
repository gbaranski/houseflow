package auth

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gbaranski/houseflow/pkg/token"
	"github.com/gbaranski/houseflow/pkg/types"
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

	signedCode, err := token.NewSignedFromBase64([]byte(form.Code))
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "invalid_grant",
			Description: fmt.Sprintf("authorization code %s", err.Error()),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}
	if err = signedCode.Verify([]byte(a.opts.AuthorizationCodeKey)); err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "invalid_grant",
			Description: fmt.Sprintf("authorization code %s", err.Error()),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}
	parsed := signedCode.Parse()

	rt, err := a.newRefreshToken(parsed.Audience)
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "rt_create_fail",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(json)
		return
	}

	at, err := a.newAccessToken(parsed.Audience)
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
		AccessToken:  string(at.Base64()),
		RefreshToken: string(rt.Base64()),
		ExpiresIn:    int(token.AccessTokenDuration.Seconds()),
	})
	w.Write(json)
}

func (a *Auth) onRefreshTokenGrant(w http.ResponseWriter, r *http.Request, form TokenQuery) {
	signedRT, err := token.NewSignedFromBase64WithVerify([]byte(a.opts.RefreshKey), []byte(form.RefreshToken))
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "invalid_grant",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	signedAT, err := a.newAccessToken(signedRT.Parse().Audience)
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
		AccessToken: string(signedAT.Base64()),
		ExpiresIn:   int(token.AccessTokenDuration.Seconds()),
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
