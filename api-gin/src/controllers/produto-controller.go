package controllers

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
	produto_service "tcc-api-gin/src/services"
	"tcc-api-gin/src/utils"
)

func GetAllProdutos(ctx *gin.Context) {
	page, pageSize := getParams(ctx)
	produtos, restErr := produto_service.GetAllProdutos(page, pageSize)
	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Produtos obtidos com sucesso",
		http.StatusOK,
		produtos,
	))
}

func GetProdutoBySKU(ctx *gin.Context) {
	sku := ctx.Param("sku")
	produto, restErr := produto_service.GetProdutoBySku(sku)

	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Produto obtido com sucesso",
		http.StatusOK,
		produto,
	))

}

func GetProdutoByCategoria(ctx *gin.Context) {
	categoria := ctx.Param("categoria")
	page, pageSize := getParams(ctx)
	produtos, restErr := produto_service.GetProdutoByCategoria(categoria, page, pageSize)

	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Produtos obtido com sucesso",
		http.StatusOK,
		produtos,
	))

}

func GetAllOfertas(ctx *gin.Context) {
	page, pageSize := getParams(ctx)
	produtos, restErr := produto_service.GetAllOfertas(page, pageSize)
	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Ofertas obtidas com sucesso",
		http.StatusOK,
		produtos,
	))
}

func GetAllDestaques(ctx *gin.Context) {
	page, pageSize := getParams(ctx)
	produtos, restErr := produto_service.GetAllDestaques(page, pageSize)
	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Destaques obtidos com sucesso",
		http.StatusOK,
		produtos,
	))
}

func GetProdutoByName(ctx *gin.Context) {
	page, pageSize := getParams(ctx)
	name := ctx.DefaultQuery("name", "")
	if name == "" {
		restErr := utils.NewRestErr(400, "name deve ser enviado por query param", nil)
		utils.RespondRestErr(restErr, ctx)
		return
	}

	produtos, restErr := produto_service.GetProdutoByName(name, page, pageSize)
	if restErr != nil {
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.JSON(http.StatusOK, utils.NewAppMessage(
		"Produtos obtidos com sucesso",
		http.StatusOK,
		produtos,
	))
}

func getParams(ctx *gin.Context) (int, int) {
	page, err := strconv.Atoi(ctx.DefaultQuery("page", "1"))
	if err != nil || page < 1 {
		page = 1
	}

	pageSize, err := strconv.Atoi(ctx.DefaultQuery("pageSize", "10"))
	if err != nil || pageSize <= 0 {
		pageSize = 10

	}

	return page, pageSize
}
