package auth

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/mongo"
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
		if err == mongo.ErrNoDocuments {
			http.Error(w, "Invalid username or password", http.StatusUnauthorized)
			return
		}
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	//compare the user from the request, with the one we defined:
	if dbUser.Email != creds.Email {
		http.Error(w, "Invalid username or password", http.StatusUnauthorized)
		return
	}
	passmatch := utils.ComparePasswordAndHash([]byte(creds.Password), []byte(dbUser.Password))
	if !passmatch {
		http.Error(w, "Invalid username or password", http.StatusUnauthorized)
		return
	}
	redirectURI, err := a.createRedirectURI(query, dbUser.ID)
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
		merr := err.(mongo.WriteException)
		for _, werr := range merr.WriteErrors {
			if werr.Code == 11000 {
				http.Error(w, "Account already exists", http.StatusConflict)
				return
			}
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	}

	w.Write([]byte(fmt.Sprintf("Account created with ID: %s, now you can log in", id.Hex())))
}
