package database

import (
	"errors"
	"fmt"
	"reflect"
	"strings"

	utils "github.com/gbaranski/houseflow/auth/utils"
)

// User struct, used to either creating user, or this one in DB
type User struct {
	FirstName string `json:"firstName" bson:"firstName"`
	LastName  string `json:"lastName" bson:"lastName"`
	Email     string `json:"email" bson:"email" houseflow:"email"`
	Password  string `json:"password" bson:"password"`
}

// Validate validates password, returns error
func (u *User) Validate() error {
	fields := reflect.ValueOf(u).Elem()
	for i := 0; i < fields.NumField(); i++ {
		field := fields.Field(i)
		structField := fields.Type().Field(i)

		if field.IsZero() {
			return fmt.Errorf("Field %s is missing", structField.Name)
		}

		houseflowTags := structField.Tag.Get("houseflow")
		if strings.Contains(houseflowTags, "email") && !utils.IsEmailValid(field.String()) {
			return errors.New("Invalid email")
		}
	}
	return nil
}

func (u *User) hashPassword() error {
	bytes, err := utils.HashPassword(u.Password)
	if err != nil {
		return err
	}
	u.Password = string(bytes)
	return nil
}
