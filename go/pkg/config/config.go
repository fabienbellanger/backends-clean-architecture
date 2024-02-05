package config

import (
	"github.com/spf13/viper"
)

type Config struct {
	// Server
	URL  string
	PORT string

	// Database
	DB_DRIVER   string
	DB_HOST     string
	DB_USERNAME string
	DB_PASSWORD string
	DB_PORT     string
	DB_DATABASE string
}

var C Config

func InitConfig() error {
	viper.SetConfigFile(".env")

	if err := viper.ReadInConfig(); err != nil {
		return err
	}

	if err := viper.Unmarshal(&C); err != nil {
		return err
	}

	return nil
}
