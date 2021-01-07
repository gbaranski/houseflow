package utils

import (
	"testing"
)

func TestGenerateRandomString(t *testing.T) {
	for i := 10; i < 100; i++ {
		var strings [10]string
		for range strings {
			random := GenerateRandomString(i)
			if len(random) != i {
				t.Fatalf("random string length doesn't match, expected: %d, received: %d\n", i, len(random))
			}
			for _, v := range strings {
				if v == random {
					t.Fatalf("no enough randomness at length %d", i)
				}
			}
		}
	}
}
