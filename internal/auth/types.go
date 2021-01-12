package auth

// LoginPageQuery sent by google
type LoginPageQuery struct {
	ClientID     string `form:"client_id" binding:"required" url:"client_id"`
	RedirectURI  string `form:"redirect_uri" binding:"required" url:"redirect_uri"`
	State        string `form:"state" binding:"required" url:"state"`
	Scope        string `form:"scope" url:"scope"`
	ResponseType string `form:"response_type" binding:"required" url:"response_type"`
	UserLocale   string `form:"user_locale" url:"user_locale"`
}

// TokenQuery sent by google
type TokenQuery struct {
	ClientID     string `form:"client_id" binding:"required"`
	ClientSecret string `form:"client_secret" binding:"required"`
	GrantType    string `form:"grant_type" binding:"required"`
	Code         string `form:"code" binding:"required_if=GrantType authorization_code"`
	RedirectURI  string `form:"redirect_uri" binding:"required_if=GrantType authorization_code"`
	RefreshToken string `form:"refresh_token" binding:"required_if=GrantType refresh_token"`
}

// LoginRequest is body of the request sent to /login
type LoginRequest struct {
	Email    string `form:"email"`
	Password string `form:"password"`
}
