package utils

import (
	"net/http"
	"strings"
)

// ExtractAuthorizationToken extracts auth token
func ExtractAuthorizationToken(r *http.Request) string {
	reqToken := r.Header.Get("Authorization")
	splitToken := strings.Split(reqToken, "Bearer ")
	return splitToken[1]
}
