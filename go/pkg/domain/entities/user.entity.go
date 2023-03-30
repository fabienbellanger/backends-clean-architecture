package entities

import (
	"github.com/google/uuid"
	"strings"
	"time"
)

// User entity
type User struct {
	ID        uuid.UUID
	Lastname  string
	Firstname string
	Email     string
	Password  string
	CreatedAt time.Time
}

// NewUser creates a new user entity.
func NewUser(id uuid.UUID, lastname, firstname, email, password string, createdAt time.Time) User {
	return User{
		ID:        id,
		Lastname:  lastname,
		Firstname: firstname,
		Email:     email,
		Password:  password,
		CreatedAt: createdAt,
	}
}

// Fullname returns user full name.
// TODO: Add test
func (u *User) Fullname() string {
	return strings.TrimSpace(u.Firstname + " " + u.Lastname)
}
