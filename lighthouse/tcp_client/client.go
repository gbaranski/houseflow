package tcp_client

import (
  "net"
  "fmt"
  "bytes"

  "github.com/gbaranski/houseflow/lighthouse/packets"
  "github.com/sirupsen/logrus"
)


type ExecuteHandler = func(packets.ExecutePayload) packets.ExecuteResponsePayload

type Config struct {
  Host string
  Port uint16

  ExecuteHandler ExecuteHandler
}

type Client struct {
  conn net.Conn
  cfg Config

  CloseChannel chan struct{}
}

func Connect(cfg Config) (Client, error) {
  conn, err := net.Dial("tcp", fmt.Sprintf("%s:%d", cfg.Host, cfg.Port))
  if err != nil {
    return Client{}, err
  }

  client := Client {
    conn: conn,
  }
  go client.readLoop()

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

  switch opcode {
    case packets.OpCodeConnACK:
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
