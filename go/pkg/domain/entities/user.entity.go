package entities

import (
	vo "clean-architecture/pkg/domain/value_objects"
	"strings"
	"time"

	"github.com/google/uuid"
)

type UserID = uuid.UUID

func NewUserID() UserID {
	return uuid.New()
}

func UserIDFromString(id string) UserID {
	return uuid.MustParse(id)
}

// User entity
type User struct {
	ID        UserID
	Lastname  string
	Firstname string
	Email     vo.Email
	Password  vo.Password
	CreatedAt time.Time
}

// NewUser creates a new user entity
func NewUser(id UserID, lastname, firstname, email, password string, createdAt time.Time) User {
	return User{
		ID:        id,
		Lastname:  lastname,
		Firstname: firstname,
		Email:     vo.NewEmail(email),
		Password:  vo.NewPassword(password),
		CreatedAt: createdAt,
	}
}

// Fullname returns user full name
func (u *User) Fullname() string {
	return strings.TrimSpace(u.Firstname + " " + u.Lastname)
}
