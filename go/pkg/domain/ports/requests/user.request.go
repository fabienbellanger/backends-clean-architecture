package requests

import (
	"clean-architecture/pkg/domain/entities"
	"clean-architecture/utils"
	"time"

	"github.com/google/uuid"
)

// UserCreateRequest is the type provide in the request.
type UserCreateRequest struct {
	Lastname  string `json:"lastname" xml:"lastname" form:"lastname" validate:"required"`
	Firstname string `json:"firstname" xml:"firstname" form:"firstname" validate:"required"`
	Email     string `json:"email" xml:"email" form:"email" validate:"required,email"`
	Password  string `json:"-" xml:"-" form:"password" validate:"required,min=8"`
}

// ToUserEntity transforms a UserCreateRequest into a User entity.
func (uc *UserCreateRequest) ToUserEntity() entities.User {
	return entities.NewUser(
		uuid.New(),
		uc.Lastname,
		uc.Firstname,
		uc.Email,
		uc.Password,
		time.Now(),
	)
}

// Validate request input data.
func (uc *UserCreateRequest) Validate() utils.ValidatorErrors {
	return utils.ValidateStruct(uc)
}

// GetUserRequest is the type provide in the request.
type GetUserRequest struct {
	ID string `json:"id" xml:"id" form:"id" validate:"uuid4,required"`
}

// Validate request input data.
func (uc *GetUserRequest) Validate() utils.ValidatorErrors {
	return utils.ValidateStruct(uc)
}
