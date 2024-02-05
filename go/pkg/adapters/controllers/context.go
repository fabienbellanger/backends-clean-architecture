package controllers

type Context interface {
	JSON(data interface{}) error
	Bind(i interface{}) error
	Query(key string, defaultValue ...string) string
	Body(out interface{}) error
	GetResponseHeader(key string, defaultValue ...string) string
	GetRequestHeader(key string, defaultValue ...string) string
	GetRequestHeaders() map[string]string
}
