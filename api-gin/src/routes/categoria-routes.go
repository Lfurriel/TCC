package routes

import (
	"github.com/gin-gonic/gin"
	controller "tcc-api-gin/src/controllers"
)

func CategoriaRoutes(router *gin.RouterGroup) {
	categoria := router.Group("/categorias")
	{
		categoria.GET("/", controller.GetAllCategorias)
	}
}
