package controllers

import (
	"github.com/gin-gonic/gin"
	"net/http"
	categoria_service "tcc-api-gin/src/services"
	"tcc-api-gin/src/utils"
)

func GetAllCategorias(ctx *gin.Context) {
	categorias, restErr := categoria_service.GetAllCategorias()

	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Categorias obtidas com sucesso",
		http.StatusOK,
		categorias,
	))
}
