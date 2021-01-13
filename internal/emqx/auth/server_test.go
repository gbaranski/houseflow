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

var a Auth
var pkey, skey, _ = ed25519.GenerateKey(rand.Reader)

var opts = Options{
	ServerPublicKey: pkey,
}

type TestDatabase struct {
	Devices []types.Device
}

func (tdb TestDatabase) GetDeviceByID(ctx context.Context, id primitive.ObjectID) (types.Device, error) {
	for _, d := range tdb.Devices {
		if d.ID == id {
			return d, nil
		}
	}
	return types.Device{}, mongo.ErrNoDocuments
}

func TestMain(m *testing.M) {
	a = New(TestDatabase{}, opts)

	os.Exit(m.Run())
}
