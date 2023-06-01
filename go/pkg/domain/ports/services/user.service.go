package services

import (
	"clean-architecture/pkg/domain/entities"
	"clean-architecture/pkg/domain/ports/repositories"
	"clean-architecture/pkg/domain/ports/requests"
)

type userService struct {
	userRepository repositories.UserRepository
}

// NewUserService returns a new user service.
func NewUserService(repo repositories.UserRepository) UserService {
	return &userService{repo}
}

type UserService interface {
	Create(r *requests.UserCreateRequest) (*entities.User, error)
	GetUser(r *requests.GetUserRequest) (*entities.User, error)
}

func (us userService) Create(req *requests.UserCreateRequest) (*entities.User, error) {
	// Validation
	if err := req.Validate(); err != nil {
		return nil, err
	}

	// Save user in database
	user := req.ToUserEntity()
	err := us.userRepository.CreateUser(&user)

	return &user, err
}

func (us userService) GetUser(req *requests.GetUserRequest) (*entities.User, error) {
	// Validation
	if err := req.Validate(); err != nil {
		return nil, err
	}

	// Get user from ID
	user, err := us.userRepository.GetUser(req.ID)

	return &user, err
}
