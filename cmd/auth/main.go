package main

import (
	"context"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"time"

	"github.com/caarlos0/env/v6"

	"github.com/gbaranski/houseflow/internal/auth"
	"github.com/gbaranski/houseflow/pkg/database/postgres"
)

func main() {
	log.Println("Starting auth service")
	var (
		postgresOptions postgres.Options
		authOptions     auth.Options
	)
	if err := env.Parse(&postgresOptions); err != nil {
		panic(fmt.Errorf("fail load postgres opts %s", err.Error()))
	}
	if err := env.Parse(&authOptions); err != nil {
		panic(fmt.Errorf("fail load auth opts %s", err.Error()))
	}
	loginSiteTemplate, err := template.ParseFiles("/templates/auth.tmpl")
	if err != nil {
		panic("fail load login site template")
	}
	authOptions.LoginSiteTemplate = loginSiteTemplate

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	postgres, err := postgres.New(ctx, postgresOptions)
	if err != nil {
		panic(err)
	}

	auth := auth.New(postgres, authOptions)

	http.ListenAndServe(":80", auth.Router)
	if err != nil {
		panic(err)
	}
}
