package dal

import (
	"net/http"
	"tcc-api-gin/src/configs"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func CreatePedido(pedido *models.Pedido) (*models.Pedido, *utils.RestErr) {
	if err := configs.DB.Create(pedido).Error; err != nil {
		return nil, utils.NewRestErr(http.StatusNotFound, "Erro ao criar pedido", err)
	}
	return pedido, nil
}
