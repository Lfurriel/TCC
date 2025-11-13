package main

import (
	"github.com/gin-gonic/gin"
	"log"
	"os"
	"tcc-api-gin/src/configs"
	middleware "tcc-api-gin/src/middlewares"
	"tcc-api-gin/src/routes"
)

func init() {
	configs.LoadEnvVariables()
	configs.ConnectToDb()
	configs.BindingValidator()
}

func main() {
	r := gin.Default()
	r.Use(middleware.ErrorHandlingMiddleware())

	routes.RegisterRoutes(r)

	port := os.Getenv("PORT")
	if port == "" {
		port = "3333"
	}

	// Iniciar o servidor
	log.Printf("Servidor rodando na porta %s...", port)
	r.Run(":" + port)
}
