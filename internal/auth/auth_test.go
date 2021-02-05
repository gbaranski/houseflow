package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"strconv"
	"strings"
	"testing"

	"github.com/gbaranski/houseflow/pkg/token"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/google/uuid"
)

const (
	// bcrypt hashed "helloworld"
	helloworld = "$2y$12$sVtI/bYDQ3LWKcGlryQYzeo3IFjIYsl4f4bY6isfBaE3MnaPIcc2e"
)

var userID = uuid.New().String()
var opts = Options{
	ProjectID:            "houseflow",
	ClientID:             "someRandomClientID",
	ClientSecret:         "someRandomClientSecret",
	AccessKey:            "someRandomAccessKey",
	AuthorizationCodeKey: "someRandomAuthorizationCodeKey",
	RefreshKey:           "someRandomRefreshKey",
}
var a Auth

var realUser = types.User{
	ID:           userID,
	FirstName:    "John",
	LastName:     "Smith",
	Email:        "john.smith@gmail.com",
	PasswordHash: []byte(helloworld),
}

type TestDatabase struct{}

func (db TestDatabase) AddUser(ctx context.Context, user types.User) (string, error) {
	return uuid.New().String(), nil
}

func (db TestDatabase) GetUserByEmail(ctx context.Context, email string) (*types.User, error) {
	if email == realUser.Email {
		return &realUser, nil
	}
	return nil, nil
}

func TestMain(m *testing.M) {
	a = New(TestDatabase{}, opts)
	os.Exit(m.Run())
}

func TestLoginWithoutBody(t *testing.T) {
	w := httptest.NewRecorder()
	req, _ := http.NewRequest("POST", "/login", nil)
	a.Router.ServeHTTP(w, req)
	if w.Code != http.StatusBadRequest {
		t.Fatalf("unexpected /login response %d", w.Code)
	}
}

func TestLoginInvalidPassword(t *testing.T) {
	q := LoginPageQuery{
		ClientID:     opts.ClientID,
		RedirectURI:  fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", opts.ProjectID),
		State:        utils.GenerateRandomString(20),
		ResponseType: "code",
		UserLocale:   "en_US",
	}
	query := url.Values{}
	encoder.Encode(q, query)

	form := LoginCredentials{
		Email:    realUser.Email,
		Password: "jgjnjsnjgfnfsnfgngsfndkngf", // invalid password
	}
	data := url.Values{}
	err := encoder.Encode(form, data)
	if err != nil {
		panic(err)
	}

	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, fmt.Sprintf("/login?%s", query.Encode()), strings.NewReader(data.Encode()))
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Add("Content-Length", strconv.Itoa(len(data.Encode())))
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Fatalf("unexpected /login response %d", w.Code)
	}
}

func TestLoginValidPassword(t *testing.T) {
	lpQuery := url.Values{}
	encoder.Encode(LoginPageQuery{
		ClientID:     opts.ClientID,
		RedirectURI:  fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", opts.ProjectID),
		State:        utils.GenerateRandomString(20),
		ResponseType: "code",
		UserLocale:   "en_US",
	}, lpQuery)

	form := LoginCredentials{
		Email:    realUser.Email,
		Password: "helloworld", // valid password
	}
	data := url.Values{}
	err := encoder.Encode(form, data)
	if err != nil {
		panic(err)
	}

	w := httptest.NewRecorder()
	req, err := http.NewRequest(http.MethodPost, fmt.Sprintf("/login?%s", lpQuery.Encode()), strings.NewReader(data.Encode()))
	if err != nil {
		panic(err)
	}
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Add("Content-Length", strconv.Itoa(len(data.Encode())))
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusSeeOther {
		t.Fatalf("unexpected /login response %d", w.Code)
	}
	redirectURL := w.Header()["Location"][0]
	url, err := url.Parse(redirectURL)
	if err != nil {
		t.Fatalf("redirected URL is invalid: %s", redirectURL)
	}
	signedcode, err := token.NewSignedFromBase64([]byte(url.Query().Get("code")))
	if err != nil {
		t.Fatal(err)
	}
	if err = signedcode.Verify([]byte(opts.AuthorizationCodeKey)); err != nil {
		t.Fatalf("fail verify authorization code %s", err.Error())
	}
	res := GetAuthorizationCodeGrant(t, signedcode)
	GetRefreshTokenGrant(t, res.RefreshToken)
}

func GetAuthorizationCodeGrant(t *testing.T, signedcode token.Signed) AuthorizationCodeGrantResponse {
	form := url.Values{}
	codeb64 := signedcode.Base64()
	encoder.Encode(TokenQuery{
		ClientID:     opts.ClientID,
		ClientSecret: opts.ClientSecret,
		GrantType:    "authorization_code",
		Code:         string(codeb64[:]),
		RedirectURI:  fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", opts.ProjectID),
	}, form)

	w := httptest.NewRecorder()
	req, err := http.NewRequest(http.MethodPost, "/token", strings.NewReader(form.Encode()))
	if err != nil {
		panic(err)
	}
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Add("Content-Length", strconv.Itoa(len(form.Encode())))
	a.Router.ServeHTTP(w, req)

	if w.Code != 200 {
		t.Fatalf("authorization code grant unexpected response: %d, body: %s", w.Code, w.Body.String())
	}
	var res AuthorizationCodeGrantResponse
	if err = json.Unmarshal(w.Body.Bytes(), &res); err != nil {
		t.Fatalf("fail decode body response, err: %s, body: %s", err.Error(), w.Body.String())
	}

	signedAT, err := token.NewSignedFromBase64([]byte(res.AccessToken))
	if err != nil {
		t.Fatalf("fail parse access token %s", err.Error())
	}
	if err := signedAT.Verify([]byte(opts.AccessKey)); err != nil {
		t.Fatalf("fail verify access token %s", err.Error())
	}

	signedRT, err := token.NewSignedFromBase64([]byte(res.RefreshToken))
	if err != nil {
		t.Fatalf("fail parse refresh token %s", err.Error())
	}
	if err := signedRT.Verify([]byte(opts.RefreshKey)); err != nil {
		t.Fatalf("fail verify refresh token %s", err.Error())
	}
	return res
}

func GetRefreshTokenGrant(t *testing.T, refreshToken string) RefreshTokenGrantResponse {
	form := url.Values{}
	encoder.Encode(TokenQuery{
		ClientID:     opts.ClientID,
		ClientSecret: opts.ClientSecret,
		GrantType:    "refresh_token",
		RefreshToken: refreshToken,
	}, form)

	w := httptest.NewRecorder()
	req, err := http.NewRequest(http.MethodPost, "/token", strings.NewReader(form.Encode()))
	if err != nil {
		panic(err)
	}
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Add("Content-Length", strconv.Itoa(len(form.Encode())))
	a.Router.ServeHTTP(w, req)

	if w.Code != 200 {
		t.Fatalf("refresh token grant unexpected response: %d, body: %s", w.Code, w.Body.String())
	}
	var res RefreshTokenGrantResponse
	if err = json.Unmarshal(w.Body.Bytes(), &res); err != nil {
		t.Fatalf("fail decode body response, err: %s, body: %s", err.Error(), w.Body.String())
	}

	signedAT, err := token.NewSignedFromBase64([]byte(res.AccessToken))
	if err != nil {
		t.Fatalf("fail parse access token %s", err.Error())
	}
	if signedAT.Verify([]byte(opts.AccessKey)); err != nil {
		t.Fatalf("fail verify access token %s", err.Error())
	}
	return res
}
