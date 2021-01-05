type UserRequest struct {
	ClientID string `json:"clientid"`
	IP       string `json:"ip"`
	Username string `json:"username"`
	Password string `json:"password"`
}

type ACLRequest struct {
	Access   string `json:"access"`
	ClientID string `json:"clientid"`
	IP       string `json:"ip"`
	Username string `json:"username"`
	Password string `json:"password"`
}
