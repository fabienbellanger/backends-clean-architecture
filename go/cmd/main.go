package main

import (
	"clean-architecture/pkg/config"
	"fmt"
	"os"
)

func main() {
	fmt.Println("Go backend using clean architecture")

	err := config.InitConfig()
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	fmt.Printf("%v\n", config.C)
}
