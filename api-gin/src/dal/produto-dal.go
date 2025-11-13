package dal

import (
	"errors"
	"fmt"
	"gorm.io/gorm"
	"net/http"
	"tcc-api-gin/src/configs"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func GetAllProdutos(campos string, page int, pageSize int) (*[]models.Produto, *utils.RestErr) {
	var produtos []models.Produto

	if pageSize > 100 {
		pageSize = 100
	}

	offset := (page - 1) * pageSize

	camposComCategoria := fmt.Sprintf("%s, categorias.id as \"Categoria__id\", categorias.nome as \"Categoria__nome\"", campos)
	query := configs.DB.Select(camposComCategoria).Joins("JOIN categorias ON produtos.\"idCategoria\" = categorias.id").
		Offset(offset).Limit(pageSize)

	if err := query.Find(&produtos).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return &produtos, nil
		}
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar produtos", err)
	}

	return &produtos, nil
}

func GetRandomProdutos(campos string, limite int) ([]models.Produto, *utils.RestErr) {
	var produtos []models.Produto

	err := configs.DB.Select(campos).Where("RANDOM() < ?", 0.01).
		Preload("Categoria", func(db *gorm.DB) *gorm.DB {
			return db.Select("id, nome")
		}).Limit(limite).Find(&produtos).Error

	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return produtos, nil
		}
		return nil, utils.NewRestErr(
			http.StatusInternalServerError,
			"Erro ao buscar produtos",
			err,
		)
	}

	return produtos, nil
}

func GetAllProdutosBySKU(skus []string, campos string) ([]models.Produto, *utils.RestErr) {
	produtos := new([]models.Produto)
	err := configs.DB.Select(campos).Find(&produtos, skus).Error
	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar produtos", err)
	}
	return *produtos, nil
}

func GetAllProdutosByCategoria(categoria string, campos string, page, pageSize int) (*[]models.Produto, *utils.RestErr) {
	var produtos []models.Produto

	offset := (page - 1) * pageSize
	camposComCategoria := fmt.Sprintf("%s, categorias.id as \"Categoria__id\", categorias.nome as \"Categoria__nome\"", campos)
	query := configs.DB.Select(camposComCategoria).Joins("JOIN categorias ON produtos.\"idCategoria\" = categorias.id").
		Where("\"idCategoria\" = ?", categoria).Offset(offset).Limit(pageSize)

	if err := query.Find(&produtos).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, utils.NewRestErr(http.StatusNotFound, "Produtos não encontrados", err)
		}
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar produtos", err)
	}

	return &produtos, nil
}

func GetBySKU(sku string, campos string) (*models.Produto, *utils.RestErr) {
	var produto models.Produto

	camposComCategoria := fmt.Sprintf("%s, categorias.id as \"Categoria__id\", categorias.nome as \"Categoria__nome\"", campos)
	err := configs.DB.Select(camposComCategoria).Joins("JOIN categorias ON produtos.\"idCategoria\" = categorias.id").
		Where("sku = ?", sku).First(&produto).Error

	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, utils.NewRestErr(http.StatusNotFound, "Produto não encontrado", err)
		}
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar produto", err)
	}

	return &produto, nil
}

func UpdateProduto(produto models.Produto) *utils.RestErr {
	if err := configs.DB.Save(&produto).Error; err != nil {
		return utils.NewRestErr(http.StatusInternalServerError, "Erro ao atualizar estoque", err)
	}

	return nil
}

func GetAllOfertas(campos string, page int, pageSize int) (*[]models.Produto, *utils.RestErr) {
	var produtos []models.Produto

	offset := (page - 1) * pageSize

	camposComCategoria := fmt.Sprintf("%s, categorias.id as \"Categoria__id\", categorias.nome as \"Categoria__nome\"", campos)

	query := configs.DB.Select(camposComCategoria).Joins("JOIN categorias ON produtos.\"idCategoria\" = categorias.id").
		Where("pctoferta > 0").Order("pctoferta DESC").Offset(offset).Limit(pageSize)

	if err := query.Find(&produtos).Error; err != nil {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar produtos", err)
	}

	return &produtos, nil
}

func GetAllDestaques(campos string, page, pageSize int) (*[]models.Produto, *utils.RestErr) {
	var produtos []models.Produto

	if page < 1 {
		page = 1
	}
	if pageSize <= 0 {
		pageSize = 10
	}

	offset := (page - 1) * pageSize

	camposComCategoria := fmt.Sprintf("%s, categorias.id as \"Categoria__id\", categorias.nome as \"Categoria__nome\"", campos)
	query := configs.DB.Select(camposComCategoria).Joins("JOIN categorias ON produtos.\"idCategoria\" = categorias.id").
		Order("pctoferta DESC").Offset(offset).Limit(pageSize)

	if err := query.Find(&produtos).Error; err != nil {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar produtos", err)
	}

	return &produtos, nil
}

func GetProdutoByName(campos string, nome string, page int, pageSize int) ([]models.Produto, *utils.RestErr) {
	produtos := new([]models.Produto)
	err := configs.DB.Select(campos).Where("nome ILIKE ?", "%"+nome+"%").Limit(pageSize).
		Offset((page - 1) * pageSize).Find(&produtos).Error

	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar produtos", err)
	}
	return *produtos, nil
}
