package entities

import (
	"strings"
	"time"
)

// User entity
type User struct {
	ID        string
	Lastname  string
	Firstname string
	Username  string
	Password  string
	CreatedAt time.Time
}

// NewUser creates a new user entity.
func NewUser(id, lastname, firstname, username, password string, createdAt time.Time) User {
	return User{
		ID:        id,
		Lastname:  lastname,
		Firstname: firstname,
		Username:  username,
		Password:  password,
		CreatedAt: createdAt,
	}
}

// Fullname returns user fullname.
// TODO: Add test
func (u *User) Fullname() string {
	return strings.TrimSpace(u.Firstname + " " + u.Lastname)
}
