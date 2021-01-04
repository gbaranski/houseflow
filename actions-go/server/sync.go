package server

import (
	"github.com/gbaranski/houseflow/actions/fulfillment"
	"github.com/gbaranski/houseflow/actions/types"
)

// OnSync handles sync intent https://developers.google.com/assistant/smarthome/reference/intent/sync
func (s *Server) onSync(r fulfillment.SyncRequest, user types.User) {

}
