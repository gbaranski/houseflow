package utils

import (
	"fmt"
	"regexp"

	"golang.org/x/crypto/bcrypt"
)

var emailRegex = regexp.MustCompile("^[a-zA-Z0-9.!#$%&'*+\\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$")

// IsEmailValid checks if the email provided passes the required structure and length.
func IsEmailValid(e string) bool {
	if len(e) < 3 && len(e) > 254 {
		return false
	}
	return emailRegex.MatchString(e)
}

// HashPassword hashes password with bcrypt
func HashPassword(pass string) ([]byte, error) {
	fmt.Println("Received to hashing this: ", []byte(pass))
	one, err := bcrypt.GenerateFromPassword([]byte(pass), bcrypt.DefaultCost)
	two, err := bcrypt.GenerateFromPassword([]byte(pass), bcrypt.DefaultCost)
	fmt.Println("one: ", one)
	fmt.Println("two: ", two)
	return one, err
}

// CheckPasswordHash checks hash of password
func CheckPasswordHash(password, hash string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hash), []byte(password))
	return err == nil
}
