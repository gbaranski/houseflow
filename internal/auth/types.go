package auth

// LoginPageQuery sent by google
type LoginPageQuery struct {
	ClientID     string `form:"client_id" binding:"required"`
	RedirectURI  string `form:"redirect_uri" binding:"required"`
	State        string `form:"state" binding:"required"`
	Scope        string `form:"scope"`
	ResponseType string `form:"response_type" binding:"required"`
	UserLocale   string `form:"user_locale"`
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
