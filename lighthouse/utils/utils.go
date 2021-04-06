package utils

import (
	"encoding/binary"
	"fmt"
	"io"
)

// Read16BitInteger reads 16 bit integer from bytes reader
func Read16BitInteger(r io.Reader) (uint16, error) {
	bytes := make([]byte, 2)
	_, err := r.Read(bytes)
	if err != nil {
		return 0, err
	}
	return binary.BigEndian.Uint16(bytes), nil
}

// ReadLength ...
func ReadLength(r io.Reader) (uint32, error) {
	len := uint32(0)

	buf := make([]byte, 1)
	buf[0] = 0b10000000

	for (buf[0] >> 7) == 1 {
		buf = make([]byte, 1)
		_, err := r.Read(buf)
		if err != nil {
			return 0, err
		}
		len += uint32(buf[0] & 0b01111111)
	}

	return len, nil
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
