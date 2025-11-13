package main

import (
	"tcc-api-gin/src/configs"
	"tcc-api-gin/src/models"
)

func init() {
	configs.LoadEnvVariables()
	configs.ConnectToDb()
}

func main() {
	configs.DB.AutoMigrate(&models.Categoria{})
	configs.DB.AutoMigrate(&models.Cliente{})
	configs.DB.AutoMigrate(&models.Pedido{})
	configs.DB.AutoMigrate(&models.ProdutoPedido{})
	configs.DB.AutoMigrate(&models.EnderecoEntrega{})
	configs.DB.AutoMigrate(&models.Pagamento{})
	configs.DB.AutoMigrate(&models.Produto{})
}
