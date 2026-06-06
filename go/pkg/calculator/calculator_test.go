package calculator

import (
	"testing"
)

func TestNoteName(t *testing.T) {
	tests := []struct {
		x, y     int
		expected string
	}{
		{49, 55, "Cb77v+"},
		{31, 16, "Sorry - that is a 31-limit pitch!"},
		{3, 2, "G"},
		{5, 4, "E"},
		{7, 4, "Bb7"},
		{11, 8, "F^"},
		{13, 8, "Ab3"},
	}

	for _, tt := range tests {
		result := NoteName(tt.x, tt.y, true)
		if result != tt.expected {
			t.Errorf("NoteName(%d, %d) = %q, expected %q", tt.x, tt.y, result, tt.expected)
		}
	}
}
