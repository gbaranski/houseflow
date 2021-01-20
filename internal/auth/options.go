package auth

import "html/template"

// Options defines options for auth
type Options struct {
	// ProjectID set in Google Cloud Console
	//
	// *Required*
	ProjectID string `env:"PROJECT_ID,required"`

	// ClientID set Actions account linking tab
	//
	// *Required*
	ClientID string `env:"OAUTH_CLIENT_ID,required"`

	// ClientSecret set Actions account linking tab
	//
	// *Required*
	ClientSecret string `env:"OAUTH_CLIENT_SECRET,required"`

	// AccessKey is secret for signing access tokens
	//
	// *Required*
	AccessKey string `env:"ACCESS_KEY,required"`

	// AuthorizationCodeKey is secret for signing authorization codes
	//
	// *Required*
	AuthorizationCodeKey string `env:"AUTHORIZATION_CODE_KEY,required"`

	// RefreshKey is secret for signing refresh tokens
	//
	// *Required*
	RefreshKey string `env:"REFRESH_KEY,required"`

	// Template of login site
	//
	// *Required*
	LoginSiteTemplate *template.Template
}
