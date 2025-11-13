package controllers

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"tcc-api-gin/src/models"
	pedido_service "tcc-api-gin/src/services"
	"tcc-api-gin/src/utils"
	"tcc-api-gin/src/validations"
)

func CreatePedido(ctx *gin.Context) {
	cliente, exists := ctx.Get("cliente")
	if !exists {
		restErr := utils.NewRestErr(http.StatusUnauthorized, "Usuário não autenticado", nil)
		utils.RespondRestErr(restErr, ctx)
		return
	}

	clienteId := cliente.(string)

	var pedido models.Pedido
	if !validations.PedidoValido(&pedido, clienteId, ctx) {
		return
	}

	pedidoResult, restErr := pedido_service.CreatePedido(&pedido)

	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusCreated, utils.NewAppMessage(
		"Pedido criado com sucesso",
		http.StatusOK,
		pedidoResult,
	))

}
