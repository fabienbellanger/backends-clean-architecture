package entities

import (
	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func TestUserFullname(t *testing.T) {
	u := NewUser(uuid.New(), "Doe", "John", "john.doe@test.com", "00000000", time.Now())

	assert.Equal(t, "John Doe", u.Fullname())
}

func TestUserFullnameWithoutLastname(t *testing.T) {
	u := NewUser(uuid.New(), "", "John", "john.doe@test.com", "00000000", time.Now())

	assert.Equal(t, "John", u.Fullname())
}

func TestUserFullnameWithoutFirstname(t *testing.T) {
	u := NewUser(uuid.New(), "Doe", "", "john.doe@test.com", "00000000", time.Now())

	assert.Equal(t, "Doe", u.Fullname())
}

func TestUserFullnameWithoutLastnameAndFirstname(t *testing.T) {
	u := NewUser(uuid.New(), "", "", "john.doe@test.com", "00000000", time.Now())

	assert.Equal(t, "", u.Fullname())
}
