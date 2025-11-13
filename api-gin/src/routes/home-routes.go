package routes

import (
	"github.com/gin-gonic/gin"
	controller "tcc-api-gin/src/controllers"
)

func HomeRoutes(router *gin.RouterGroup) {
	home := router.Group("/home")
	{
		home.GET("/amazon", controller.GetAmazonHome)
		home.GET("/shopee", controller.GetShoppeHome)
	}
}
