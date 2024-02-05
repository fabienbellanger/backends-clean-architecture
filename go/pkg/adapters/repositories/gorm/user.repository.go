package gorm

import (
	"clean-architecture/pkg/adapters/repositories/gorm/models"
	"clean-architecture/pkg/domain/entities"
	"clean-architecture/pkg/domain/ports/repositories"

	"gorm.io/gorm"
)

type userOrmRepository struct {
	db *gorm.DB
}

func NewUserOrmRepository(db *gorm.DB) repositories.UserRepository {
	return &userOrmRepository{db}
}

// CreateUser creates a new user.
func (uor *userOrmRepository) CreateUser(u *entities.User) error {
	user := models.UserFromEntity(u)
	return uor.db.Create(user).Error
}

// CreateUser creates a new user.
func (uor *userOrmRepository) GetUser(id string) (entities.User, error) {
	var user models.User
	if err := uor.db.Find(&user).Error; err != nil {
		return entities.User{}, err
	}

	return user.UserToEntity(), nil
}
