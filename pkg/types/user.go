package types

// User struct, used to either creating user, or this one in DB
type User struct {
	ID           string `json:"id" form:"-"`
	FirstName    string `json:"firstName" form:"firstName" binding:"required"`
	LastName     string `json:"lastName" form:"lastName" binding:"required"`
	Email        string `json:"email" houseflow:"email" form:"email" binding:"required,email"`
	Password     string `json:"password" form:"password" binding:"required,min=8,max=20"`
	PasswordHash []byte `json:"-" form:"-"`
}
