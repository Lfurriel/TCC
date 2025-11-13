package routes

import (
	"github.com/gin-gonic/gin"
	controller "tcc-api-gin/src/controllers"
	middleware "tcc-api-gin/src/middlewares"
)

func PedidoRoutes(router *gin.RouterGroup) {
	pedido := router.Group("/pedidos")
	{
		pedido.POST("/", middleware.IsAuthenticated, controller.CreatePedido)
	}
}
