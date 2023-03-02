package user

import "clean-architecture/domain/entities"

type CreatePresenter interface {
	DisplayUser(user entities.User)
}
