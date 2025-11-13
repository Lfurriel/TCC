package routes

import (
	"github.com/gin-gonic/gin"
	controller "tcc-api-gin/src/controllers"
)

func ProdutoRoutes(router *gin.RouterGroup) {
	produto := router.Group("/produtos")
	{
		produto.GET("/:sku", controller.GetProdutoBySKU)
		produto.GET("/categoria/:categoria", controller.GetProdutoByCategoria)
		produto.GET("/", controller.GetAllProdutos)
		produto.GET("/ofertas", controller.GetAllOfertas)
		produto.GET("/destaques", controller.GetAllDestaques)
		produto.GET("/nome", controller.GetProdutoByName)
	}
}
