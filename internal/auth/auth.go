package auth

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
)

func (a *Auth) onLogin(w http.ResponseWriter, r *http.Request) {
	if err := r.ParseForm(); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	var creds LoginCredentials
	if err := decoder.Decode(&creds, r.PostForm); err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		return
	}

	var query LoginPageQuery
	if err := decoder.Decode(&query, r.URL.Query()); err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		return
	}

	dbUser, err := a.db.GetUserByEmail(r.Context(), creds.Email)
	if err != nil {
		http.Error(w, "Invalid username or password", http.StatusUnauthorized)
		return
	}

	//compare the user from the request, with the one we defined:
	if dbUser.Email != creds.Email {
		http.Error(w, "Invalid username or password", http.StatusUnauthorized)
		return
	}

	passmatch := utils.ComparePasswordAndHash(creds.Password, dbUser.PasswordHash)
	if !passmatch {
		http.Error(w, "Invalid username or password", http.StatusUnauthorized)
		return
	}
	redirectURI, err := a.createRedirectURI(query, []byte(dbUser.ID))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	http.Redirect(w, r, redirectURI, http.StatusSeeOther)

}

func (a *Auth) onRegister(w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	if err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		return
	}

	var query LoginPageQuery
	if err = decoder.Decode(&query, r.URL.Query()); err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		return
	}

	var newUser types.User
	if err = decoder.Decode(&newUser, r.PostForm); err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	id, err := a.db.AddUser(ctx, newUser)
	if err != nil {
		http.Error(w, "Account already exists", http.StatusConflict)
		return
	}

	w.Write([]byte(fmt.Sprintf("Account created with ID: %s, now you can log in", id)))
}
