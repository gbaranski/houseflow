package auth

import (
	"context"
	"crypto/ed25519"
	"crypto/rand"
	"os"
	"testing"

	"github.com/gbaranski/houseflow/pkg/types"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
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

func (tdb TestDatabase) GetDeviceByID(ctx context.Context, id string) (types.Device, error) {
	did, _ := primitive.ObjectIDFromHex(id)
	for _, d := range devices {
		if d.ID == did {
			return d, nil
		}
	}
	return types.Device{}, mongo.ErrNoDocuments
}

func TestMain(m *testing.M) {
	a = New(TestDatabase{}, opts)

	os.Exit(m.Run())
}
