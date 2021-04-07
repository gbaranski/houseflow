package server

import (
  "testing"
	"github.com/google/uuid"
)

func TestStore(t *testing.T) {
  store := NewSessionStore()
  session := Session {
    ClientID: uuid.Must(uuid.NewRandom()),
    conn: nil,
  }

  if err := store.Add(&session); err != nil {
    t.Fatalf("fail adding to store: %s", err.Error())
  }
  if !store.Exists(session.ClientID) {
    t.Fatalf("session does not exists in store after adding")
  }
  retrievedSession := store.Get(session.ClientID)
  if retrievedSession == nil {
    t.Fatalf("retrieving session failed")
  }
  if retrievedSession != &session {
    t.Fatalf("retrieved session pointer mismatch, expect: %s, got: %s", &session, retrievedSession)
  }
  store.Delete(session.ClientID)
  if store.Exists(session.ClientID) {
    t.Fatalf("session exists in store after deleting")
  }
}


func BenchmarkAddGetStore(b *testing.B) {
  store := NewSessionStore()
  session := Session {
    ClientID: uuid.Must(uuid.NewRandom()),
    conn: nil,
  }
  for i := 0; i < 1000; i++ {
    session := Session {
      ClientID: uuid.Must(uuid.NewRandom()),
      conn: nil,
    }
    store.Add(&session)
  }
  b.ResetTimer()

  for i := 0; i < b.N; i++ {
    store.Add(&session)
    store.Get(session.ClientID)
    store.Delete(session.ClientID)
  }
}
