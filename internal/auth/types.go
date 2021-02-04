package auth

import "net/url"

// LoginPageQuery sent by google
type LoginPageQuery struct {
	ClientID     string `form:"client_id" binding:"required" url:"client_id"`
	RedirectURI  string `form:"redirect_uri" binding:"required" url:"redirect_uri"`
	State        string `form:"state" binding:"required" url:"state"`
	Scope        string `form:"scope" url:"scope"`
	ResponseType string `form:"response_type" binding:"required" url:"response_type"`
	UserLocale   string `form:"user_locale" url:"user_locale"`
}

// DecodeLoginPageQuery decodes login page query from url query
func DecodeLoginPageQuery(u url.Values) LoginPageQuery {
	return LoginPageQuery{
		ClientID:     u.Get("client_id"),
		RedirectURI:  u.Get("redirect_uri"),
		State:        u.Get("state"),
		Scope:        u.Get("scope"),
		ResponseType: u.Get("response_type"),
		UserLocale:   u.Get("user_locale"),
	}
}

// TokenQuery sent by google
type TokenQuery struct {
	ClientID     string `schema:"client_id" binding:"required"`
	ClientSecret string `schema:"client_secret" binding:"required"`
	GrantType    string `schema:"grant_type" binding:"required"`
	Code         string `schema:"code" binding:"required_if=GrantType authorization_code"`
	RedirectURI  string `schema:"redirect_uri" binding:"required_if=GrantType authorization_code"`
	RefreshToken string `schema:"refresh_token" binding:"required_if=GrantType refresh_token"`
}

// LoginCredentials is body of the request sent to /login
type LoginCredentials struct {
	Email    string `form:"email"`
	Password string `form:"password"`
}

// AuthorizationCodeGrantResponse is struct which you get in response for authorization code grant if everything is successful
type AuthorizationCodeGrantResponse struct {
	TokenType    string `json:"token_type"`
	AccessToken  string `json:"access_token"`
	RefreshToken string `json:"refresh_token"`
	// In seconds
	ExpiresIn int `json:"expires_in"`
}

// RefreshTokenGrantResponse is struct which you get in response for refresh code grant if everything is successful
type RefreshTokenGrantResponse struct {
	TokenType   string `json:"token_type"`
	AccessToken string `json:"access_token"`
	// In seconds
	ExpiresIn int `json:"expires_in"`
}
