package services

import (
	"tcc-api-gin/src/dal"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func GetAllCategorias() (*[]models.Categoria, *utils.RestErr) {
	return dal.GetAllCategoria()
}
