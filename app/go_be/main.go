package main

import (
	"encoding/json"
	"log"
	"net/http"
)

type Response struct {
	Message string `json:"message"`
}

func whoami(w http.ResponseWriter, r *http.Request) {
	response := Response{
		Message: "Hello, World!",
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(response)
}

func main() {
	http.HandleFunc("/go/whoami", whoami)
	log.Fatal(http.ListenAndServe(":8900", nil))
}
