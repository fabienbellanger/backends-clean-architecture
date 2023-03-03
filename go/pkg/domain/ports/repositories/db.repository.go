package repositories

type DBRepository interface {
	Transaction(func(interface{}) (interface{}, error)) (interface{}, error)
}
