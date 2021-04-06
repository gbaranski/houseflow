package server

import "testing"

func TestClientStore(t *testing.T) {
	cs := NewClientStore()
	c1 := Client{
		ID:        "SomeClientID",
		IPAddress: nil,
	}
	c2 := Client{
		ID:        "SomeDifferentClientID",
		IPAddress: nil,
	}

	if err := cs.Add(c1); err != nil {
		t.Fatalf("fail add c1 %s", err.Error())
	}
	if err := cs.Add(c2); err != nil {
		t.Fatalf("fail add c2 %s", err.Error())
	}

	c, ok := cs.Get(c1.ID)
	if !ok {
		t.Fatalf("fail retreive c1")
	}
	if c.ID != c1.ID {
		t.Fatalf("invalid c1 clientID")
	}

	c, ok = cs.Get(c2.ID)
	if !ok {
		t.Fatalf("fail retreive c2")
	}
	if c.ID != c2.ID {
		t.Fatalf("invalid c2 clientID")
	}
}

func TestClientStoreNotPresentUser(t *testing.T) {
	cs := NewClientStore()
	c1 := Client{
		ID:        "SomeClientID",
		IPAddress: nil,
	}

	c2 := Client{
		ID:        "SomeDifferentClientID",
		IPAddress: nil,
	}

	if err := cs.Add(c1); err != nil {
		t.Fatalf("fail add c1 %s", err.Error())
	}

	c, ok := cs.Get(c1.ID)
	if !ok {
		t.Fatalf("fail retreive c1")
	}
	if c.ID != c1.ID {
		t.Fatalf("invalid c1 clientID")
	}

	c, ok = cs.Get(c2.ID)
	if ok {
		t.Fatalf("expected to fail on c2")
	}
	if c.ID == c2.ID {
		t.Fatalf("invalid c2 clientID")
	}
}

func TestClientStoreDuplicate(t *testing.T) {
	cs := NewClientStore()
	c1 := Client{
		ID:        "SomeClientID",
		IPAddress: nil,
	}

	if err := cs.Add(c1); err != nil {
		t.Fatalf("fail add c1 %s", err.Error())
	}

	if err := cs.Add(c1); err == nil {
		t.Fatalf("expected error, received nil while adding dup")
	}
}
