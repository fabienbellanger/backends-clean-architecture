package user

import (
	"clean-architecture/domain/entities"
	"clean-architecture/domain/presenters/user"
	"clean-architecture/domain/repositories"
)

// CreateUser use case.
type CreateUser struct {
	Repository repositories.UserRepository
	User       entities.User
}

// NewCreateUser returns a new CreateUser use case.
func NewCreateUser(repo repositories.UserRepository, user entities.User) CreateUser {
	return CreateUser{
		Repository: repo,
		User:       user,
	}
}

// Execute create user use case.
func (uc *CreateUser) Execute(presenter user.CreatePresenter) {
	// Save user in database
	uc.Repository.CreateUser(uc.User)

	// API response
	presenter.DisplayUser(uc.User)
}
