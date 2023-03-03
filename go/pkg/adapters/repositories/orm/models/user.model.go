package models

import (
	"clean-architecture/pkg/domain/entities"
	"time"
)

// User models.
type User struct {
	ID        string    `json:"id" xml:"id" form:"id" gorm:"primaryKey" validate:"required,uuid"`
	Lastname  string    `json:"lastname" xml:"lastname" form:"lastname" gorm:"size:63" validate:"required"`
	Firstname string    `json:"firstname" xml:"firstname" form:"firstname" gorm:"size:63" validate:"required"`
	Username  string    `json:"username" xml:"username" form:"username" gorm:"not null;unique;size:127" validate:"required,email"`
	Password  string    `json:"-" xml:"-" form:"password" gorm:"not null;index;size:128" validate:"required,min=8"`
	CreatedAt time.Time `json:"created_at" xml:"created_at" form:"created_at" gorm:"not null;autoCreateTime"`
}

// UserToEntity converts a user model into a domain user entity.
// TODO: Add test
func (u *User) UserToEntity() entities.User {
	return entities.NewUser(
		u.ID,
		u.Lastname,
		u.Firstname,
		u.Username,
		u.Password,
		u.CreatedAt,
	)
}

// UserFromEntity converts a user entity into a user model.
func UserFromEntity(u *entities.User) User {
	return User{
		u.ID,
		u.Lastname,
		u.Firstname,
		u.Username,
		u.Password,
		u.CreatedAt,
	}
}
