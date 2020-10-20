package services

import (
	"context"
	"errors"
	"log"

	"cloud.google.com/go/firestore"
	firebase "firebase.google.com/go"
	types "github.com/gbaranski/houseflow/webhooks/types"
)

// FirebaseClient firebaseClient type
type FirebaseClient struct {
	db *firestore.Client
}

// FirebaseUserDevices refers to user devices in database
type FirebaseUserDevices struct {
	uid string
}

// FirebaseUser refers to user in database
type FirebaseUser struct {
	devices  []FirebaseUserDevices
	role     string
	uid      string
	username string
}

// FirebaseDevice refers to device in firebase
type FirebaseDevice struct {
	IPAddress  string
	Status     bool
	DeviceType string
	UID        string
}

// InitFirebase firebase initialization
func InitFirebase(ctx context.Context) (*FirebaseClient, error) {
	app, err := firebase.NewApp(ctx, nil)
	if err != nil {
		log.Fatalf("error initializing app: %v\n", err)
	}
	db, err := app.Firestore(ctx)
	if err != nil {
		return nil, err
	}
	return &FirebaseClient{
		db: db,
	}, nil
}

// UpdateDeviceStatus update device status
func UpdateDeviceStatus(ctx context.Context, client *FirebaseClient, uid string, status bool) {
	_, err := client.db.Collection("devices").Doc(uid).Set(ctx, map[string]interface{}{
		"status": status,
	}, firestore.MergeAll)
	if err != nil {
		log.Printf("Error ocurred when updating status %s\n", err)
		return
	}
	log.Printf("Success updating device %s status to %t \n", uid, status)
}

// GetUserUsername retreives user username from firestore
func GetUserUsername(ctx context.Context, client *FirebaseClient, uid string) (string, error) {
	dsnap, err := client.db.Collection("users").Doc(uid).Get(ctx)
	if err != nil {
		return "error", err
	}
	if dsnap.Exists() == false {
		return "error", errors.New("firebase: FirebaseUser snapshot does not exist")
	}
	data := dsnap.Data()
	return data["username"].(string), nil
}

// GetFirebaseDevice retreives firebase device from firestore
func GetFirebaseDevice(ctx context.Context, client *FirebaseClient, uid string) (*FirebaseDevice, error) {
	dsnap, err := client.db.Collection("users").Doc(uid).Get(ctx)
	if err != nil {
		return nil, err
	}
	if dsnap.Exists() == false {
		return nil, errors.New("firebase: FirebaseUser snapshot does not exist")
	}
	data := dsnap.Data()
	firebaseDevice := FirebaseDevice{IPAddress: data["type"].(string), Status: data["status"].(bool), DeviceType: data["type"].(string), UID: data["uid"].(string)}
	return &firebaseDevice, nil

}

// AddDeviceHistory adds history for a device
func AddDeviceHistory(ctx context.Context, client *FirebaseClient, firebaseDevice *FirebaseDevice, deviceRequest *types.DeviceRequest) (string, error) {
	ref, _, err := client.db.Collection("devices").Doc(firebaseDevice.UID).Collection("history").Add(ctx, map[string]interface{}{
		"request":    deviceRequest.Request,
		"username":   deviceRequest.Username,
		"ipAddress":  deviceRequest.IPAddress,
		"timestamp":  deviceRequest.Timestamp,
		"deviceUid":  firebaseDevice.UID,
		"deviceType": firebaseDevice.DeviceType,
	})
	return ref.ID, err
}
