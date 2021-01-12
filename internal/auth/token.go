package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func (a *Auth) onTokenAuthorizationCodeGrant(w http.ResponseWriter, r *http.Request, form TokenQuery) {
	if !a.validateRedirectURI(form.RedirectURI) {
		json, _ := json.Marshal(map[string]interface{}{
			"error": "invalid_redirect_uri",
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	token, err := utils.VerifyToken(form.Code, []byte(a.opts.AuthorizationCodeKey))
	if err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "invalid_grant",
			"error_description": fmt.Sprintf("authorization code %s", err.Error()),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	userID, err := primitive.ObjectIDFromHex(token.Audience)
	if err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	rt, rtstr, err := a.newRefreshToken(userID)
	if err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "rt_create_fail",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(json)
		return
	}

	_, atstr, err := a.newAccessToken(userID)
	if err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "at_create_fail",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(json)
		return
	}

	err = a.redis.AddToken(r.Context(), userID, rt)

	if err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "fail_add_rt",
			"error_description": err.Error(),
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
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	userID, err := a.redis.FetchToken(ctx, *rt)
	if err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}

	userIDObject, err := primitive.ObjectIDFromHex(userID)
	if err != nil {
		fmt.Println("Unable to parse userID to objectID")
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}
	_, atstr, err := a.newAccessToken(userIDObject)
	if err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "fail_create_at",
			"error_description": err.Error(),
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
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "fail_parse_form",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write(json)
		return
	}

	var form TokenQuery
	if err = decoder.Decode(&form, r.PostForm); err != nil {
		json, _ := json.Marshal(map[string]interface{}{
			"error":             "fail_parse_form",
			"error_description": err.Error(),
		})
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write(json)
		return
	}

	if form.ClientID != a.opts.ClientID || form.ClientSecret != a.opts.ClientSecret {
		json, _ := json.Marshal(map[string]interface{}{
			"error": "invalid_oauth_credentials",
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
		json, _ := json.Marshal(map[string]interface{}{
			"error": "unknown_grant_type",
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
	}
}
