package controllers

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"tcc-api-gin/src/services"
	"tcc-api-gin/src/utils"
)

func GetAmazonHome(ctx *gin.Context) {
	homeAmazon, restErr := services.GetAmazonHome()

	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Home obtida com sucesso",
		http.StatusOK,
		homeAmazon,
	))
}

func GetShoppeHome(ctx *gin.Context) {
	homeShopee, restErr := services.GetShoppeHome()

	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Home obtida com sucesso",
		http.StatusOK,
		homeShopee,
	))
}
