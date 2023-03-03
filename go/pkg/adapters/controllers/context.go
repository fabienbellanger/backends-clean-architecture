package controllers

type Context interface {
	JSON(data interface{}) error
	Bind(i interface{}) error
	Query(vars interface{}) string
}
