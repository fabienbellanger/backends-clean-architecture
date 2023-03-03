package config

import (
	"github.com/spf13/viper"
)

type config struct {
	DB_DRIVER   string
	DB_HOST     string
	DB_USERNAME string
	DB_PASSWORD string
	DB_PORT     string
	DB_DATABASE string
}

var C config

func InitConfig() error {
	viper.SetConfigFile(".env")

	if err := viper.ReadInConfig(); err != nil {
		return err
	}

	Config := &C
	if err := viper.Unmarshal(&Config); err != nil {
		return err
	}

	return nil
}
