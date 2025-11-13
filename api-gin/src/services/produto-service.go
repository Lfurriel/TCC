package services

import (
	"tcc-api-gin/src/dal"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func GetAllProdutos(page int, pageSize int) (*[]models.Produto, *utils.RestErr) {
	return dal.GetAllProdutos("*", page, pageSize)
}

func GetProdutoBySku(sku string) (*models.Produto, *utils.RestErr) {
	return dal.GetBySKU(sku, "sku, produtos.nome, foto, pctOferta, preco, descricao, estoque")
}

func GetProdutoByCategoria(categoria string, page int, pageSize int) (*[]models.Produto, *utils.RestErr) {
	return dal.GetAllProdutosByCategoria(categoria, "sku, produtos.nome, foto, pctOferta, preco, qtdVendas", page, pageSize)
}

func GetAllOfertas(page int, pageSize int) (*[]models.Produto, *utils.RestErr) {
	return dal.GetAllOfertas("sku, produtos.nome, foto, pctOferta, preco", page, pageSize)
}

func GetAllDestaques(page int, pageSize int) (*[]models.Produto, *utils.RestErr) {
	return dal.GetAllDestaques("sku, produtos.nome, foto, pctOferta, preco", page, pageSize)
}

func GetProdutoByName(nome string, page int, pageSize int) ([]models.Produto, *utils.RestErr) {
	return dal.GetProdutoByName("sku, produtos.nome, foto, pctOferta, preco", nome, page, pageSize)
}
