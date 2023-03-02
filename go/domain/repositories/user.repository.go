package repositories

import "clean-architecture/domain/entities"

// UserRepository interface.
type UserRepository interface {
	CreateUser(user entities.User) error
	GetUser(id string) (entities.User, error)
}
