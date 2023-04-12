package utils

import (
	"fmt"

	"github.com/go-playground/validator/v10"
)

// ValidatorError represents error validation struct.
type ValidatorError struct {
	FailedField string
	Tag         string
	Value       string
}

// TODO: Change
func (ve ValidatorError) Error() string {
	return "Field: %s, Tag: %s, Value: %s"
}

// ValidatorErrors is a slice of ValidatorError.
type ValidatorErrors []ValidatorError

// TODO: Change
func (ve ValidatorErrors) Error() string {
	e := ""
	for _, v := range ve {
		e += fmt.Sprintf("%s\n", v)
	}
	return e
}

// ValidateStruct checks if a struct is valid and returns an array of errors
// if it is not valid.
func ValidateStruct(task interface{}) (errors ValidatorErrors) {
	validate := validator.New()
	err := validate.Struct(task)
	if err != nil {
		for _, err := range err.(validator.ValidationErrors) {
			errors = append(errors, ValidatorError{
				FailedField: err.Field(),
				Tag:         err.Tag(),
				Value:       err.Param(),
			})
		}
	}
	return
}
