package orm

import "clean-architecture/domain/entities"

// UserOrmRepository implements domain UserRepository for ORM (gorm).
type UserOrmRepository struct {
	// db interface{} // TODO: Use real DB
}

// CreateUser creates a new user.
func (uor *UserOrmRepository) CreateUser(user entities.User) error {
	// uor.db.Create(user)...
	return nil
}

// GetUser returns a user from its ID.
func (uor *UserOrmRepository) GetUser(id string) (entities.User, error) {
	// uor.db.Select(id)...
	return entities.User{}, nil
}
