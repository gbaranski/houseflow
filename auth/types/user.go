package types

import (
	"errors"
	"fmt"
	"os"
	"reflect"
	"strings"
	"time"

	"github.com/dgrijalva/jwt-go"
	"github.com/gbaranski/houseflow/auth/utils"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// User struct, used to either creating user, or this one in DB
type User struct {
	ID        primitive.ObjectID `bson:"_id,omitempty" json:"id,omitempty" houseflow:"ignore"`
	FirstName string             `json:"firstName" bson:"firstName"`
	LastName  string             `json:"lastName" bson:"lastName"`
	Email     string             `json:"email" bson:"email" houseflow:"email"`
	Password  string             `json:"password" bson:"password"`
}

// Validate validates password, returns error
func (u *User) Validate() error {
	fields := reflect.ValueOf(u).Elem()
	for i := 0; i < fields.NumField(); i++ {
		field := fields.Field(i)
		structField := fields.Type().Field(i)

		houseflowTags := structField.Tag.Get("houseflow")

		if field.IsZero() && !strings.Contains(houseflowTags, "ignore") {
			return fmt.Errorf("Field %s is missing", structField.Name)
		}

		if strings.Contains(houseflowTags, "email") && !utils.IsEmailValid(field.String()) {
			return errors.New("Invalid email")
		}
	}
	return nil
}

// CreateAccessToken creates JWT access token for user
func (u *User) CreateAccessToken() (*string, error) {
	key := os.Getenv("AUTH_JWT_KEY")
	atClaims := jwt.MapClaims{
		"authorized": true,
		"user_id":    u.ID,
		"exp":        time.Now().Add(time.Minute * 15).Unix(),
	}
	at := jwt.NewWithClaims(jwt.SigningMethodHS256, atClaims)
	token, err := at.SignedString([]byte(key))
	if err != nil {
		return nil, err
	}
	return &token, nil
}
