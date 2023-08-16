package controllers

import (
	"clean-architecture/pkg/domain/entities"
	"clean-architecture/pkg/domain/ports/requests"
	"clean-architecture/pkg/domain/usecases"
)

type userController struct {
	userUsecase usecases.User
}

// User controller interface
type User interface {
	GetUser(c Context) error
	CreateUser(c Context) error
}

// NewUserController creates a new User controller
func NewUserController(us usecases.User) User {
	return &userController{us}
}

// CreateUser controller
func (uc *userController) CreateUser(ctx Context) error {
	var params requests.UserCreateRequest
	if err := ctx.Bind(&params); err != nil {
		return err
	}

	user, err := uc.userUsecase.Create(&params)
	if err != nil {
		return err
	}

	return ctx.JSON(user)
}

// GetUser controller
func (uc *userController) GetUser(ctx Context) error {
	var user *entities.User
	req := requests.GetUserRequest{ID: ctx.Query("id")}

	user, err := uc.userUsecase.GetUser(&req)
	if err != nil {
		return err
	}

	return ctx.JSON(user)
}
