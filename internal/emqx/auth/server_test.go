package auth

import (
	"context"
	"crypto/ed25519"
	"crypto/rand"
	"os"
	"testing"

	"github.com/gbaranski/houseflow/pkg/types"
)

var (
	a                                    Auth
	serverPublicKey, serverPrivateKey, _ = ed25519.GenerateKey(rand.Reader)
	devicePublicKey, devicePrivateKey, _ = ed25519.GenerateKey(rand.Reader)

	opts = Options{
		ServerPublicKey: serverPublicKey,
	}

	devices []types.Device
)

type TestDatabase struct {
}

func (tdb TestDatabase) GetDeviceByID(ctx context.Context, id string) (*types.Device, error) {
	for _, d := range devices {
		if d.ID == id {
			return &d, nil
		}
	}
	return nil, nil
}

func TestMain(m *testing.M) {
	a = New(TestDatabase{}, opts)

	os.Exit(m.Run())
}
