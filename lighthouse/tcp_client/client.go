package tcp_client

import (
  "net"
  "fmt"
  "bytes"

	"github.com/google/uuid"
  "github.com/gbaranski/houseflow/lighthouse/packets"
  "github.com/sirupsen/logrus"
)


type ExecuteHandler = func(packets.ExecutePayload) packets.ExecuteResponsePayload

type Config struct {
  Host string
  Port uint16
  ClientID uuid.UUID

  ExecuteHandler ExecuteHandler
}

type Client struct {
  conn net.Conn
  cfg Config
  acknowledged bool

  CloseChannel chan struct{}
}

func Connect(cfg Config) (Client, error) {
  conn, err := net.Dial("tcp", fmt.Sprintf("%s:%d", cfg.Host, cfg.Port))
  if err != nil {
    return Client{}, err
  }

  client := Client {
    conn: conn,
    cfg: cfg,
    acknowledged: false,
    CloseChannel: make(chan struct{}),
  }
  buf := bytes.NewBuffer([]byte{})
  _, err = packets.Packet{
    OpCode: packets.OpCodeConnect,
    Payload: packets.ConnectPayload{
      ClientID: cfg.ClientID,
    },
  }.WriteTo(buf)

  if err != nil {
    client.conn.Close()
    return client, err
  }

  _, err = buf.WriteTo(client.conn)
  if err != nil {
    client.conn.Close()
    return client, err
  }

  // go client.readLoop()

  return client, nil
}


func (c *Client) readLoop() {
  for {
    err := c.read()
    if err != nil {
      logrus.WithError(err).Errorln("error when reading packet")
      break
    }
  }
  c.CloseChannel<-struct{}{}
  c.conn.Close()
}

func (c *Client) read() error {
  opcode, err := packets.ReadOpCode(c.conn)
  if err != nil {
    return err
  }

  if !c.acknowledged && opcode != packets.OpCodeConnACK {
    return fmt.Errorf("expected first packet to be ConnACK")
  }
  switch opcode {
    case packets.OpCodeConnACK:
      c.acknowledged = true
      p, err := packets.ReadConnACKPayload(c.conn)
      if err != nil {
        return err
      }
      if p.ReturnCode != packets.ConnACKConnectionAccepted {
        return fmt.Errorf("unexpected return code: %x", p.ReturnCode)
      }
    case packets.OpCodeExecute:
      p, err := packets.ReadExecutePayload(c.conn)
      if err != nil {
        return err
      }
      buf := bytes.NewBuffer([]byte{})
      _, err = packets.Packet {
        OpCode: packets.OpCodeExecuteResponse,
        Payload: c.cfg.ExecuteHandler(p),
      }.WriteTo(buf)
      if err != nil {
        return err
      }
      _,  err = buf.WriteTo(c.conn)
      if err != nil {
        return err
      }
  }

  return nil
}
