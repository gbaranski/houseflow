package auth

// LoginPageQuery sent by google
type LoginPageQuery struct {
	ClientID     string `schema:"client_id" binding:"required"`
	RedirectURI  string `schema:"redirect_uri" binding:"required"`
	State        string `schema:"state" binding:"required"`
	Scope        string `schema:"scope"`
	ResponseType string `schema:"response_type" binding:"required"`
	UserLocale   string `schema:"user_locale"`
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
