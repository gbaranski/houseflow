package types

// ResponseError is used to return it for example from some HTTP route
type ResponseError struct {
	Name        string `json:"error"`
	Description string `json:"error_description,omitempty"`
	StatusCode  int    `json:"-"`
}
