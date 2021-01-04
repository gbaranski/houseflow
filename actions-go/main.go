package main

import (
	"fmt"

	"github.com/go-playground/validator/v10"
)

type test struct {
	Something string `validate:"eq=action.devices.SYNC"`
}

func main() {
	// ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	// db, err := database.CreateDatabase(ctx)
	// defer cancel()
	tst := test{
		Something: "action.devices.SYNC",
	}
	validator := validator.New()
	err := validator.Struct(tst)
	fmt.Println(err)
}
