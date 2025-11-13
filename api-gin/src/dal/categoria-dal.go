package dal

import (
	"net/http"
	"tcc-api-gin/src/configs"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func GetAllCategoria() (*[]models.Categoria, *utils.RestErr) {
	categorias := new([]models.Categoria)
	err := configs.DB.Find(&categorias).Error

	if err != nil {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar categorias", err)
	}

	return categorias, nil
}

func GetCategoriaById(id string) (*models.Categoria, *utils.RestErr) {
	categoria := new(models.Categoria)
	err := configs.DB.Where("id = ?", id).Find(&categoria).Error

	if err != nil {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar categoria", err)
	}

	return categoria, nil
}
