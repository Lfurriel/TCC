package controllers

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"tcc-api-gin/src/models"
	cliente_service "tcc-api-gin/src/services"
	"tcc-api-gin/src/utils"
	"tcc-api-gin/src/validations"
)

func CreateCliente(ctx *gin.Context) {
	var cliente models.Cliente
	if !validations.ClienteValido(&cliente, ctx) {
		return
	}

	token, clienteResult, restErr := cliente_service.CreateCliente(cliente)
	clienteResult.Senha = ""
	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}
	ctx.JSON(http.StatusCreated, utils.NewAppMessage(
		"Login realizado com sucesso",
		http.StatusCreated,
		map[string]interface{}{
			"cliente": clienteResult,
			"token":   token,
		},
	))

}

func Login(ctx *gin.Context) {
	var login models.Login
	if !validations.LoginValido(&login, ctx) {
		return
	}
	token, cliente, restErr := cliente_service.Login(login)
	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Login realizado com sucesso",
		http.StatusOK,
		map[string]interface{}{
			"cliente": cliente,
			"token":   token,
		},
	))
}
