package server

import (
	"errors"
	"sync"

	"github.com/google/uuid"
)

// ClientStore holds map with references to sessions
type SessionStore struct {
	*sync.RWMutex
	m  map[uuid.UUID]*Session
}

// NewSessionStore creates store of sessions
func NewSessionStore() SessionStore {
	return SessionStore{
		m:  make(map[uuid.UUID]*Session),
    RWMutex: &sync.RWMutex{},
	}
}

var (
	// ErrAlreadyExists error which indicates that entry already exists in stroe
	ErrAlreadyExists = errors.New("session already exists in store")
)

// Add adds new session
func (store *SessionStore) Add(session *Session) error {
	store.Lock()
	defer store.Unlock()

	store.m[session.ClientID] = session
	return nil
}

// Get retreives session from store
func (store *SessionStore) Get(clientID uuid.UUID) *Session {
  store.RLock()
  defer store.RUnlock()

  return store.m[clientID]
}

// Delete removes session from store
func (store *SessionStore) Delete(clientID uuid.UUID) {
  store.Lock()
  defer store.Unlock()

  delete(store.m, clientID)
}


// Exusts check if session exists in store
func (store *SessionStore) Exists(clientID uuid.UUID) bool {
  store.RLock()
  defer store.RUnlock()

  _, ok := store.m[clientID]

  return ok
}

