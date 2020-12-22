package server

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	"github.com/gbaranski/houseflow/auth/database"
)

// Server hold root server state
type Server struct {
	db *database.Database
}

// CreateServer set ups server, it won't run till Server.Start
func CreateServer(db *database.Database) *Server {
	return &Server{
		db: db,
	}
}

// Start starts server, this function is blocking
func (s *Server) Start() error {
	log.Println("Starting server at port 80")
	http.HandleFunc("/createUser", s.onCreateUser)
	if err := http.ListenAndServe(":80", nil); err != nil {
		return err
	}
	return nil
}

func (s *Server) onCreateUser(w http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(w, "Method is not supported.", http.StatusNotFound)
		return
	}

	var user database.User
	err := json.NewDecoder(r.Body).Decode(&user)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if user.Validate() != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	err = s.db.AddUser(&user)
	if err != nil {
		log.Println("Error when adding user to DB: ", err)
		if database.IsDuplicateError(err) {
			http.Error(w, err.Error(), http.StatusConflict)
			return
		}

		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	fmt.Fprintf(w, "Created user: %+v", user)

}
