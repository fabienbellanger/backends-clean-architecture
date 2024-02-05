package main

import (
	"clean-architecture/pkg/config"
	"clean-architecture/pkg/infrastructure/router"
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
	fmt.Printf("Configutation: %+v\n", config.C)

	err = router.Start(&config.C)
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
}
