package services

import (
	"math"
	"net/http"
	"tcc-api-gin/src/dal"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
	"time"
)

func CreatePedido(pedido *models.Pedido) (*models.Pedido, *utils.RestErr) {
	var skuProdutos []string
	for _, p := range pedido.Produtos {
		skuProdutos = append(skuProdutos, p.SKUProduto)
	}

	pedido.ValorBruto = 0
	pedido.ValorDesconto = 0

	if len(skuProdutos) == 0 {
		return nil, utils.NewRestErr(http.StatusBadRequest, "Nenhum produto informado", nil)
	}

	lstProdutos, restErr := dal.GetAllProdutosBySKU(skuProdutos, "*")
	if restErr != nil {
		return nil, restErr
	}

	mapProdutos := make(map[string]models.Produto)
	for _, produto := range lstProdutos {
		mapProdutos[produto.SKU] = produto
	}

	if len(skuProdutos) != len(lstProdutos) {
		for _, sku := range skuProdutos {
			if _, exists := mapProdutos[sku]; !exists {
				return nil, utils.NewRestErr(http.StatusBadRequest, "Produto com SKU '"+sku+"' não existe", nil)
			}
		}
	}

	valorFrete, exists := utils.MapFrete[pedido.Endereco.CodigoIbgeUF]
	if !exists {
		return nil, utils.NewRestErr(http.StatusBadRequest, "Código IBGE UF inválido", nil)
	}

	for i := range pedido.Produtos {
		produto := &pedido.Produtos[i]

		produtoBanco, _ := mapProdutos[produto.SKUProduto]

		// Verifica estoque
		if produtoBanco.Estoque < produto.Quantidade {
			return nil, utils.NewRestErr(http.StatusBadRequest, "Produto com SKU '"+produto.SKUProduto+"' está com estoque em falta", nil)
		}

		produto.ValorUnitario = produtoBanco.Preco
		produto.ValorBruto = produto.Quantidade * produto.ValorUnitario

		desconto := math.Round(produto.ValorBruto*(*pedido.Pagamento.PorcentagemDesconto)*100) / 100
		produto.ValorDesconto = desconto

		produto.ValorLiquido = produto.ValorBruto - produto.ValorDesconto
		produto.ValorFrete = math.Round((valorFrete/float64(len(pedido.Produtos)))*100) / 100

		pedido.ValorBruto += produto.ValorBruto
		pedido.ValorDesconto += produto.ValorDesconto
	}

	// Definir data de entrega (3 dias úteis)
	dataEntrega := time.Now().AddDate(0, 0, 3)
	pedido.DataEntrega = dataEntrega

	// Calcular valores finais do pedido
	pedido.ValorFrete = valorFrete
	pedido.ValorLiquido = pedido.ValorBruto - pedido.ValorDesconto + pedido.ValorFrete

	// Configurar pagamento
	pedido.Pagamento.PorcentagemDesconto = nil
	pedido.Pagamento.ValorTotal = pedido.ValorLiquido
	pedido.Pagamento.ValorParcela = math.Round((pedido.Pagamento.ValorTotal/float64(pedido.Pagamento.NumeroParcelas))*100) / 100

	// Definir status inicial do pedido
	pedido.Status = "P"

	// Criar pedido no banco de dados
	var pedidoResult *models.Pedido
	pedidoResult, restErr = dal.CreatePedido(pedido)
	if restErr != nil {
		return nil, restErr
	}

	// Atualizar estoque dos produtos e quantidade de vendas
	for _, produto := range pedido.Produtos {
		produtoBanco, _ := mapProdutos[produto.SKUProduto]
		produtoBanco.Estoque -= produto.Quantidade
		produtoBanco.QtdVendas += produto.Quantidade
		restErr := dal.UpdateProduto(produtoBanco)
		if restErr != nil {
			return nil, restErr
		}
	}

	return pedidoResult, nil
}
