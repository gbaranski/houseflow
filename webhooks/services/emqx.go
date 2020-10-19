package services

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os"

	types "github.com/gbaranski/houseflow/webhooks/types"
	utils "github.com/gbaranski/houseflow/webhooks/utils"
)

// GetClientData returns client data
func GetClientData(clientID string) (*types.GetClientResponse, error) {
	username := os.Getenv("EMQX_USERNAME")
	password := os.Getenv("EMQX_PASSWORD")

	client := &http.Client{}
	req, err := http.NewRequest("GET", "http://emqx:8081/api/v4/clients/"+clientID, nil)
	req.SetBasicAuth(username, password)
	resp, err := client.Do(req)

	if err != nil {
		return nil, err
	}
	body, err := ioutil.ReadAll(resp.Body)
	fmt.Println(string(body))
	return utils.BytesToClientData(body)
}
