package requests

import (
	"clean-architecture/pkg/domain/entities"
	"time"

	"github.com/google/uuid"
)

// UserCreateRequest is the type provide in the request.
type UserCreateRequest struct {
	Lastname  string `json:"lastname" xml:"lastname" form:"lastname" validate:"required"`
	Firstname string `json:"firstname" xml:"firstname" form:"firstname" validate:"required"`
	Username  string `json:"username" xml:"username" form:"username" validate:"required,email"`
	Password  string `json:"-" xml:"-" form:"password" validate:"required,min=8"`
}

// ToUserEntity transforms a UserCreateRequest to User entity.
func (uc *UserCreateRequest) ToUserEntity() entities.User {
	return entities.NewUser(
		uuid.NewString(),
		uc.Lastname,
		uc.Firstname,
		uc.Username,
		uc.Password,
		time.Now(),
	)
}

// Validate request input data.
func (uc *UserCreateRequest) Validate() error {
	return nil
}

// GetUserRequest is the type provide in the request.
type GetUserRequest struct {
	ID string
}
