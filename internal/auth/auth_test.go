package auth

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var userID = primitive.NewObjectIDFromTimestamp(time.Now())
var options = Options{
	ProjectID:            "houseflow",
	ClientID:             "someRandomClientID",
	ClientSecret:         "someRandomClientSecret",
	AccessKey:            "someRandomAccessKey",
	AuthorizationCodeKey: "someRandomAuthorizationCodeKey",
	RefreshKey:           "someRandomRefreshKey",
}
var a Auth

type TestMongo struct{}

func (m TestMongo) AddUser(ctx context.Context, user types.User) (primitive.ObjectID, error) {
	return primitive.NewObjectIDFromTimestamp(time.Now()), nil
}

func (m TestMongo) GetUserByEmail(ctx context.Context, email string) (types.User, error) {
	return types.User{
		ID:        userID,
		FirstName: "John",
		LastName:  "Smith",
		Email:     email,
		Password:  "$2y$12$jKOPY8Ay3hQu2MbZ59BN2uXFouMooL.Fj0H.R.dy0YIYGhNzj4dby", // Some random password here
		Devices:   []string{},
	}, nil
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
	a = NewAuth(TestMongo{}, TestRedis{}, options)
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
