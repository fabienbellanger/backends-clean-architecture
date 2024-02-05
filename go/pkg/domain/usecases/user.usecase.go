package usecases

import (
	"clean-architecture/pkg/domain/entities"
	"clean-architecture/pkg/domain/ports/requests"
	"clean-architecture/pkg/domain/ports/services"
)

type User interface {
	Create(r *requests.UserCreateRequest) (*entities.User, error)
	GetUser(r *requests.GetUserRequest) (*entities.User, error)
}

type userUseCase struct {
	userService services.UserService
}

// NewUserUseCase returns a new CreateUser use case
func NewUserUseCase(userService services.UserService) User {
	return &userUseCase{userService}
}

// Create user
func (uc *userUseCase) Create(req *requests.UserCreateRequest) (*entities.User, error) {
	return uc.userService.Create(req)
}

func (uc *userUseCase) GetUser(req *requests.GetUserRequest) (*entities.User, error) {
	return uc.userService.GetUser(req)
}
