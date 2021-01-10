package emqxauth

// UserRequest is request about user connect
type UserRequest struct {
	ClientID string `form:"clientID"`
	IP       string `form:"ip"`
	Username string `form:"username"`
	Password string `form:"password"`
}

// ACLRequest is sub/pub req
type ACLRequest struct {
	Access     string `form:"access"`
	Username   string `form:"username"`
	ClientID   string `form:"clientid"`
	IP         string `form:"ip"`
	Topic      string `form:"topic"`
	MountPoint string `form:"mountpoint"`
}
