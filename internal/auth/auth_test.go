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
	"time"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

const (
	// bcrypt hashed "helloworld"
	helloworld = "$2y$12$sVtI/bYDQ3LWKcGlryQYzeo3IFjIYsl4f4bY6isfBaE3MnaPIcc2e"
)

var userID = primitive.NewObjectIDFromTimestamp(time.Now())
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
	ID:        userID,
	FirstName: "John",
	LastName:  "Smith",
	Email:     "john.smith@gmail.com",
	Password:  helloworld,
	Devices:   []string{},
}

type TestMongo struct{}

func (m TestMongo) AddUser(ctx context.Context, user types.User) (primitive.ObjectID, error) {
	return primitive.NewObjectIDFromTimestamp(time.Now()), nil
}

func (m TestMongo) GetUserByEmail(ctx context.Context, email string) (types.User, error) {
	if email == realUser.Email {
		return realUser, nil
	}
	return types.User{}, mongo.ErrNoDocuments
}

type TestRedis struct{}

func (r TestRedis) AddToken(ctx context.Context, userID primitive.ObjectID, token utils.Token) error {
	return nil
}
func (r TestRedis) DeleteToken(ctx context.Context, tokenID string) (int64, error) {
	return 1, nil
}

func (r TestRedis) FetchToken(ctx context.Context, token utils.Token) (string, error) {
	return userID.Hex(), nil
}

func TestMain(m *testing.M) {
	a = NewAuth(TestMongo{}, TestRedis{}, opts)
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
	redirectURL := w.HeaderMap["Location"][0]
	url, err := url.Parse(redirectURL)
	if err != nil {
		t.Fatalf("redirected URL is invalid: %s", redirectURL)
	}
	code := url.Query().Get("code")

	if _, err = utils.VerifyToken(code, []byte(opts.AuthorizationCodeKey)); err != nil {
		t.Fatalf("fail verify authorization code %s", err.Error())
	}
	GetAuthorizationCodeGrant(t, code)
}

func GetAuthorizationCodeGrant(t *testing.T, code string) {
	form := url.Values{}
	encoder.Encode(TokenQuery{
		ClientID:     opts.ClientID,
		ClientSecret: opts.ClientSecret,
		GrantType:    "authorization_code",
		Code:         code,
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
		t.Fatalf("unexpected response: %d, body: %s", w.Code, w.Body.String())
	}
	var res AuthorizationCodeGrantResponse
	if err = json.Unmarshal(w.Body.Bytes(), &res); err != nil {
		t.Fatalf("fail decode body response, err: %s, body: %s", err.Error(), w.Body.String())
	}

	if _, err = utils.VerifyToken(res.AccessToken, []byte(opts.AccessKey)); err != nil {
		t.Fatalf("fail verify access token %s", err.Error())
	}

	if _, err = utils.VerifyToken(res.RefreshToken, []byte(opts.RefreshKey)); err != nil {
		t.Fatalf("fail verify refresh token %s", err.Error())
	}

}
