package server

import (
	"errors"
	"sync"
)

// ClientStore ...
type ClientStore struct {
	c  map[string]Client
	mu sync.RWMutex
}

// NewClientStore creates store of clients
func NewClientStore() *ClientStore {
	return &ClientStore{
		c:  make(map[string]Client),
		mu: sync.RWMutex{},
	}
}

var (
	// ErrAlreadyExists error which indicates that entry already exists in stroe
	ErrAlreadyExists = errors.New("entry already exists in store")
)

// Add adds new client
func (l *ClientStore) Add(c Client) error {
	l.mu.Lock()
	defer l.mu.Unlock()
	_, ok := l.c[c.ID]
	if ok {
		return ErrAlreadyExists
	}
	l.c[c.ID] = c
	return nil
}

// Get retreives client from store
func (l *ClientStore) Get(clientID string) (Client, bool) {
	l.mu.RLock()
	defer l.mu.RUnlock()
	c, ok := l.c[clientID]
	return c, ok
}
