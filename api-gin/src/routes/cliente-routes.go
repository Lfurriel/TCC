package routes

import (
	"github.com/gin-gonic/gin"
	controller "tcc-api-gin/src/controllers"
)

func ClienteRoutes(router *gin.RouterGroup) {
	cliente := router.Group("/clientes")
	{
		cliente.POST("/", controller.CreateCliente)
		cliente.POST("/login", controller.Login)
	}
}
