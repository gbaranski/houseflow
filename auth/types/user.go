package types

import (
	"os"
	"time"

	"github.com/dgrijalva/jwt-go"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// User struct, used to either creating user, or this one in DB
type User struct {
	ID        primitive.ObjectID `bson:"_id,omitempty" json:"id,omitempty" binding:"-"`
	FirstName string             `json:"firstName" bson:"firstName" form:"firstName" binding:"required"`
	LastName  string             `json:"lastName" bson:"lastName" form:"lastName" binding:"required"`
	Email     string             `json:"email" bson:"email" houseflow:"email" form:"email" binding:"required,email"`
	Password  string             `json:"password" bson:"password" form:"password" binding:"required,min=8,max=20"`
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
