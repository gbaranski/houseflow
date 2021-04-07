package utils

import (
	"encoding/binary"
	"fmt"
	"io"
	"github.com/google/uuid"
  "math"
  "math/rand"
)

// Write16BitInteger writes 16 bit integer to bytes writer
func WriteUUID(w io.Writer, v uuid.UUID) (n int, err error) {
  b, err := v.MarshalBinary()
  if err != nil {
    return n, fmt.Errorf("fail marshalling binary")
  }

  return w.Write(b)
}

// Write16BitInteger writes 16 bit integer to bytes writer
func WriteByte(w io.Writer, v byte) (n int, err error) {
	return w.Write([]byte{v})
}

// Write16BitInteger writes 16 bit integer to bytes writer
func Write16BitInteger(w io.Writer, v uint16) (n int, err error) {
	bytes := make([]byte, 16/8)
	binary.BigEndian.PutUint16(bytes, v)
	return w.Write(bytes)
}

// Write32BitInteger writes 32 bit integer to bytes writer
func Write32BitInteger(w io.Writer, v uint32) (n int, err error) {
	bytes := make([]byte, 32/8)
	binary.BigEndian.PutUint32(bytes, v)
	return w.Write(bytes)
}

// WriteLengthPrefixedBytes writes:
// - 32 bit integer length prefix with value of len(bytes)
// - bytes
func WriteLengthPrefixedBytes(w io.Writer, b []byte) (n int, err error) {
	n, err = Write32BitInteger(w, uint32(len(b))) // Write length prefix
	if err != nil {
		return n, err
	}

	k, err := w.Write(b) // Write bytes
	if err != nil {
		return n, err
	}

	return n + k, err
}

// ReadUUID reads UUID from bytes reader
func ReadUUID(r io.Reader) (uuid.UUID, error) {
  const UUID_SIZE int = 16
	bytes := make([]byte, UUID_SIZE)
	n, err := r.Read(bytes)
	if err != nil {
		return uuid.Nil, err
	}
  if n != UUID_SIZE {
    return uuid.Nil, fmt.Errorf("invalid length: `%d`, expected: `%d`", n, UUID_SIZE)
  }

  return uuid.FromBytes(bytes)
}

// Read16BitInteger reads 16 bit integer from bytes reader
func Read16BitInteger(r io.Reader) (uint16, error) {
	bytes := make([]byte, 16/8)
	_, err := r.Read(bytes)
	if err != nil {
		return 0, err
	}
	return binary.BigEndian.Uint16(bytes), nil
}

// Read32BitInteger reads 32 bit integer from bytes reader
func Read32BitInteger(r io.Reader) (uint32, error) {
	bytes := make([]byte, 32/8)
	_, err := r.Read(bytes)
	if err != nil {
		return 0, err
	}
	return binary.BigEndian.Uint32(bytes), nil
}

// ReadLengthPrefixedString reads length prefixed slice from io.Reader
// The slice can be up to 65535 characters long, and length prefix is 16 bit integer
func ReadLengthPrefixedSlice(r io.Reader) ([]byte, error) {
	strlen, err := Read32BitInteger(r)
	if err != nil {
		return nil, fmt.Errorf("failed reading length prefix: `%s`", err.Error())
	}
	bytes := make([]byte, strlen)
	n, err := r.Read(bytes)
	if err != nil {
		return nil, err
	}
	if n != int(strlen) {
		return nil, fmt.Errorf("string has invalid length: %d, expected: %d", n, strlen)
	}

	return bytes, nil
}

// ReadByte reads single byte from io.Reader
func ReadByte(r io.Reader) (byte, error) {
	b := make([]byte, 1)
	n, err := r.Read(b)
	if err != nil {
		return 0, err
	}
	if n != 1 {
		return 0, fmt.Errorf("read invalid amount, exp: 1, n: %d", n)
	}
	return b[0], nil
}

func GenerateRandomUint16() uint16 {
  return uint16(rand.Int31n(math.MaxUint16))
}
