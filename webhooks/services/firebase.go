package services

import (
	"context"
	"log"

	"cloud.google.com/go/firestore"
	firebase "firebase.google.com/go"
)

// FirebaseClient firebaseClient type
type FirebaseClient struct {
	db *firestore.Client
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

// UpdateDeviceStatusWithIP same as UpdateDeviceStatus but also takes IP arg
func UpdateDeviceStatusWithIP(ctx context.Context, client *FirebaseClient, uid string, status bool, ip string) {
	_, err := client.db.Collection("devices").Doc(uid).Set(ctx, map[string]interface{}{
		"status": status,
		"ip":     ip,
	}, firestore.MergeAll)
	if err != nil {
		log.Printf("Error ocurred when updating status %s\n", err)
		return
	}
	log.Printf("Success updating device %s status to %t \n", uid, status)
}
