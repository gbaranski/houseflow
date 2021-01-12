package auth

import (
	"context"
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
