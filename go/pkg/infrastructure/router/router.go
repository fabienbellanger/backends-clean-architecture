package router

import (
	"clean-architecture/pkg/config"
	"fmt"
	"net/http"
)

func Start(config *config.Config) error {
	http.HandleFunc("/hello", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Go Backend using Clean Architecture")
	})

	addr := fmt.Sprintf("%s:%s", config.URL, config.PORT)
	fmt.Printf("Server running on %s...\n", addr)
	return http.ListenAndServe(addr, nil)
}
