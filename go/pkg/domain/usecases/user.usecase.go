package usecases

import (
	"clean-architecture/pkg/domain/entities"
	"clean-architecture/pkg/domain/ports/repositories"
	"clean-architecture/pkg/domain/ports/requests"
)

type userUseCase struct {
	userRepository repositories.UserRepository
}

type User interface {
	Create(r *requests.UserCreateRequest) (*entities.User, error)
	GetUser(r *requests.GetUserRequest) (*entities.User, error)
}

// NewCreateUser returns a new CreateUser use case.
func NewUserUseCase(repo repositories.UserRepository) User {
	return &userUseCase{repo}
}

// Execute create user use case.
func (uc *userUseCase) Create(req *requests.UserCreateRequest) (*entities.User, error) {
	// Validation
	if err := req.Validate(); err != nil {
		return nil, err
	}

	// Save user in database
	user := req.ToUserEntity()
	err := uc.userRepository.CreateUser(&user)

	return &user, err
}

func (uc *userUseCase) GetUser(req *requests.GetUserRequest) (*entities.User, error) {
	// Get user from ID
	user, err := uc.userRepository.GetUser(req.ID)

	return &user, err
}
