package values_objects

import "clean-architecture/utils"

// TODO: Write tests

// Password represents an password value object
type Password struct {
	value string `validate:"min=8"`
}

// Value returns the password value
func (p *Password) Value() string {
	return p.value
}

// String returns the password value
func (p *Password) String() string {
	return p.value
}

// NewPassword creates a new password
func NewPassword(value string) Password {
	return Password{value: value}
}

// Validate checks if a struct is valid and returns an array of errors
func (p *Password) Validate() utils.ValidatorErrors {
	return utils.ValidateStruct(p)
}
