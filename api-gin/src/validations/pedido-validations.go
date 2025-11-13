package validations

import (
	"errors"
	"github.com/gin-gonic/gin"
	"github.com/go-playground/validator/v10"
	"math"
	"net/http"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func PedidoValido(pedido *models.Pedido, clienteId string, ctx *gin.Context) bool {
	if err := ctx.ShouldBindJSON(&pedido); err != nil {
		var validationErrors validator.ValidationErrors
		if errors.As(err, &validationErrors) {
			var errorsList []utils.ValidationError
			for _, e := range validationErrors {
				errorsList = append(errorsList, utils.MapValidationError(e))
			}

			response := utils.NewAppMessage("Erro de Validação", http.StatusBadRequest, nil, errorsList)
			ctx.JSON(http.StatusBadRequest, response)
			return false
		}

		response := utils.NewAppMessage("Dados inválidos", http.StatusBadRequest, nil, err.Error())
		ctx.JSON(http.StatusBadRequest, response)
		return false
	}

	if !validaDesconto(pedido, ctx) {
		return false
	}

	pedido.IdCliente = clienteId
	return true
}

func validaDesconto(pedido *models.Pedido, ctx *gin.Context) bool {

	pagamento := &pedido.Pagamento

	if pagamento.PorcentagemDesconto == nil {
		valor := 0.0
		pagamento.PorcentagemDesconto = &valor
	}

	valor := *pagamento.PorcentagemDesconto
	if math.Round(valor*100)/100 != valor {
		response := utils.NewAppMessage("Erro de Validação", http.StatusBadRequest, nil, []map[string]interface{}{
			{
				"expected": "duas casas decimais",
				"message":  "O desconto deve ter no máximo duas casas decimais",
			},
		})
		ctx.JSON(http.StatusBadRequest, response)
		return false
	}

	return true
}
