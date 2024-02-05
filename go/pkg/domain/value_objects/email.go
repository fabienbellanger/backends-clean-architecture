package values_objects

import "clean-architecture/utils"

// TODO: Write tests

// Email represents an email value object
type Email struct {
	value string `validate:"email"`
}

// Value returns the email value
func (e *Email) Value() string {
	return e.value
}

// String returns the email value
func (e *Email) String() string {
	return e.value
}

// NewEmail creates a new email
func NewEmail(value string) Email {
	return Email{value: value}
}

// Validate checks if a struct is valid and returns an array of errors
func (e *Email) Validate() utils.ValidatorErrors {
	return utils.ValidateStruct(e)
}
