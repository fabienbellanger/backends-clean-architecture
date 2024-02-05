package fiber

import (
	"clean-architecture/pkg/domain/entities"
	"clean-architecture/pkg/domain/ports/requests"
	"clean-architecture/pkg/domain/usecases"

	"github.com/gofiber/fiber/v2"
)

type User interface {
	GetUser(c *fiber.Ctx) error
	CreateUser(c *fiber.Ctx) error
}

type userController struct {
	userUsecase usecases.User
}

// NewUserController creates a new User controller
func NewUserController(us usecases.User) User {
	return &userController{us}
}

// CreateUser controller
func (uc *userController) CreateUser(c *fiber.Ctx) error {
	var params requests.UserCreateRequest
	if err := c.BodyParser(&params); err != nil {
		return err
	}

	user, err := uc.userUsecase.Create(&params)
	if err != nil {
		return err
	}

	return c.JSON(user)
}

// GetUser controller
func (uc *userController) GetUser(c *fiber.Ctx) error {
	var user *entities.User
	req := requests.GetUserRequest{ID: c.Query("id")}

	user, err := uc.userUsecase.GetUser(&req)
	if err != nil {
		return err
	}

	return c.JSON(user)
}
