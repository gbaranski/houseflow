package auth

// Options defines options for auth
type Options struct {
	// ProjectID set in Google Cloud Console
	//
	// *Required*
	ProjectID string

	// ClientID set Actions account linking tab
	//
	// *Required*
	ClientID string

	// ClientSecret set Actions account linking tab
	//
	// *Required*
	ClientSecret string

	// AccessKey is secret for signing access tokens
	//
	// *Required*
	AccessKey string

	// AuthorizationCodeKey is secret for signing authorization codes
	//
	// *Required*
	AuthorizationCodeKey string

	// RefreshKey is secret for signing refresh tokens
	//
	// *Required*
	RefreshKey string
}

// Parse parses to defaults, panics if some field is required but is not present
func (opts *Options) Parse() {
	if opts.ProjectID == "" {
		panic("ProjectID must be set")
	}
	if opts.ClientID == "" {
		panic("ClientID must be set")
	}
	if opts.ClientSecret == "" {
		panic("ClientSecret must be set")
	}
}
