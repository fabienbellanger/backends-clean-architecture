package responses

type LoginResponse struct {
	ID        string `json:"id" xml:"id" form:"id" validate:"required"`
	Lastname  string `json:"lastname" xml:"lastname" form:"lastname" validate:"required"`
	Firstname string `json:"firstname" xml:"firstname" form:"firstname" validate:"required"`
	Email     string `json:"email" xml:"email" form:"email" validate:"required,email"`
	Token     string `json:"token" xml:"token" form:"token" validate:"required"`
	ExpiredAt string `json:"expired_at" xml:"expired_at" form:"expired_at" validate:"required"`
}

type GetUserResponse struct {
	ID        string `json:"id" xml:"id" form:"id" validate:"required"`
	Lastname  string `json:"lastname" xml:"lastname" form:"lastname" validate:"required"`
	Firstname string `json:"firstname" xml:"firstname" form:"firstname" validate:"required"`
	Email     string `json:"email" xml:"email" form:"email" validate:"required,email"`
	CreatedAt string `json:"created_at" xml:"created_at" form:"created_at" validate:"required"`
	UpdatedAt string `json:"updated_at" xml:"updated_at" form:"updated_at" validate:"required"`
}
