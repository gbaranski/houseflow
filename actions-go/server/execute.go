package server

import (
	"context"
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"strings"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/actions/fulfillment"
	"github.com/gbaranski/houseflow/actions/types"
	"github.com/gbaranski/houseflow/actions/utils"
	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var errDeviceTimeout = errors.New("device timeout")
var errInvalidSignature = errors.New("invalid signature")

func (s *Server) sendMQTTRequestWithResponse(ctx context.Context, device types.Device, request types.DeviceRequest) (*types.DeviceResponse, error) {
	reqTopic := fmt.Sprintf("%s/command/request", device.ID.Hex())
	resTopic := fmt.Sprintf("%s/command/response", device.ID.Hex())
	msgc := make(chan mqtt.Message)

	if token := s.mqtt.Subscribe(resTopic, 0, func(c mqtt.Client, m mqtt.Message) {
		msgc <- m
	}); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}

	defer func() {
		s.mqtt.Unsubscribe(resTopic)
	}()

	reqjson, err := json.Marshal(request)
	if err != nil {
		return nil, err
	}
	ssig := ed25519.Sign(s.config.PrivateKey, reqjson)
	encssig := base64.StdEncoding.EncodeToString(ssig)

	reqp := strings.Join([]string{encssig, string(reqjson)}, ".")

	if token := s.mqtt.Publish(reqTopic, 0, false, reqp); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}

readMessages:
	for {
		select {
		case <-ctx.Done():
			return nil, errDeviceTimeout

		case msg, ok := <-msgc:
			if !ok {
				fmt.Println("Failed waiting for msg for unknown reason")
				continue readMessages
			}

			resp := msg.Payload()
			resjson, dsig, err := utils.ParseSignedPayload(resp)
			if err != nil {
				fmt.Println("Failed parsing payload to json and sig: ", err.Error())
				continue readMessages
			}
			var res types.DeviceResponse
			err = json.Unmarshal([]byte(resjson), &res)
			if err != nil {
				fmt.Println("Failed unmarshalling json: ", err.Error())
				continue readMessages
			}
			if res.CorrelationData != request.CorrelationData {
				fmt.Println("Correlation data doesn't match, skipping")
				continue readMessages
			}
			valid := ed25519.Verify(ed25519.PublicKey(device.PublicKey), []byte(resjson), dsig)
			if !valid {
				return nil, errInvalidSignature
			}
		}
	}
}

// OnExecute https://developers.google.com/assistant/smarthome/reference/intent/execute
func (s *Server) onExecute(c *gin.Context, r fulfillment.ExecuteRequest, user types.User) {
	fmt.Println("Request: ", r)
	var deviceIDs []primitive.ObjectID
	for _, id := range user.Devices {
		objID, err := primitive.ObjectIDFromHex(id)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error":             "convert_object_id_fail",
				"error_description": err.Error(),
			})
			return
		}
		deviceIDs = append(deviceIDs, objID)
	}
	dbDevices, err := s.db.Mongo.GetUserDevices(deviceIDs)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "get_devices_fail",
			"error_description": err.Error(),
		})
		return
	}

	var responseCommands []fulfillment.ExecuteResponseCommands
	for _, command := range r.Inputs[0].Payload.Commands {
		for _, execution := range command.Execution {
			for _, device := range command.Devices {
				var correspondingDBDevice *types.Device
				for _, dbDevice := range dbDevices {
					if dbDevice.ID.Hex() == device.ID {
						correspondingDBDevice = &dbDevice
						break
					}
				}
				if correspondingDBDevice == nil {
					responseCommands = append(
						responseCommands,
						fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusError,
							ErrorCode: "relinkRequired",
						})
					continue
				}

				fmt.Println("Exeuction params: ", execution.Params)
				request := types.DeviceRequest{
					CorrelationData: utils.GenerateRandomString(16),
					State:           execution.Params,
					Command:         execution.Command,
				}
				ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
				defer cancel()
				fmt.Println(correspondingDBDevice)
				deviceResponse, err := s.sendMQTTRequestWithResponse(ctx, *correspondingDBDevice, request)
				if err != nil {
					fmt.Println("Error occured when executing on device: ", err.Error())
					if err == errDeviceTimeout {
						responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusOffline,
							ErrorCode: "offline",
							States: map[string]interface{}{
								"online": false,
							},
						})
						// should also change state in db
					} else if err == errInvalidSignature {
						responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusError,
							ErrorCode: "transientError",
							States:    correspondingDBDevice.State,
						})
					} else {
						fmt.Println("Unknown error, ", err.Error())
						responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusError,
							ErrorCode: "hardError",
							States:    correspondingDBDevice.State,
						})
					}
					continue
				}
				responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
					IDs:    []string{device.ID},
					Status: "SUCCESS",
					States: deviceResponse.State,
				})
			}
		}
	}

	response := fulfillment.ExecuteResponse{
		RequestID: r.RequestID,
		Payload: fulfillment.ExecuteResponsePayload{
			Commands: responseCommands,
		},
	}
	c.JSON(http.StatusOK, response)
}
