package server

import (
	"fmt"
	"net"

	"github.com/gbaranski/houseflow/lighthouse/packets"
	log "github.com/sirupsen/logrus"
)

func (b *Server) onConnect(conn net.Conn) (Client, error) {
	log.WithField("ip", conn.RemoteAddr().String()).Info("Starting new connection")
	_, err := packets.ReadPayloadSize(conn)
	if err != nil {
		return Client{}, fmt.Errorf("fail read payload size %s", err.Error())
	}

	cp, err := packets.ReadConnectPayload(conn)
	if err != nil {
		cack := packets.Packet{
			OpCode: packets.OpCodeConnACK,
			Payload: packets.ConnACKPayload{
				ReturnCode: packets.ConnACKMalformedPayload,
			}.Bytes(),
		}.Bytes()
		conn.Write(cack)
		return Client{}, err
	}

	cack := packets.Packet{
		OpCode: packets.OpCodeConnACK,
		Payload: packets.ConnACKPayload{
			ReturnCode: packets.ConnACKConnectionAccepted,
		}.Bytes(),
	}.Bytes()

	_, err = conn.Write(cack)
	if err != nil {
		return Client{}, err
	}

	return Client{
		ID:        cp.ClientID,
		IPAddress: conn.RemoteAddr(),
	}, nil
}
