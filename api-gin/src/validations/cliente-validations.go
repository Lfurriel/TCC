package validations

import (
	"errors"
	"github.com/gin-gonic/gin"
	"github.com/go-playground/validator/v10"
	"net/http"
	"regexp"
	"strconv"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
	"time"
)

func ClienteValido(cliente *models.Cliente, ctx *gin.Context) bool {
	if err := ctx.ShouldBindJSON(&cliente); err != nil {
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

	if err := validatePF(cliente); err != nil {
		response := utils.NewAppMessage("Erro de Validação", http.StatusBadRequest, nil, []map[string]interface{}{
			{
				"expected": "CPF",
				"message":  err.Error(),
			},
		})
		ctx.JSON(http.StatusBadRequest, response)
		return false
	}

	if err := validatePJ(cliente); err != nil {
		response := utils.NewAppMessage("Erro de Validação", http.StatusBadRequest, nil, []map[string]interface{}{
			{
				"expected": "CNPJ",
				"message":  err.Error(),
			},
		})
		ctx.JSON(http.StatusBadRequest, response)
		return false
	}

	if err := validateRazaoSocial(cliente); err != nil {
		response := utils.NewAppMessage("Erro de Validação", http.StatusBadRequest, nil, []map[string]interface{}{
			{
				"expected": "razaoSocial",
				"message":  err.Error(),
			},
		})
		ctx.JSON(http.StatusBadRequest, response)
		return false
	}

	if err := confirmaSenhasIguais(cliente); err != nil {
		response := utils.NewAppMessage("Erro de Validação", http.StatusBadRequest, nil, []map[string]interface{}{
			{
				"expected": "Senhas iguais",
				"message":  err.Error(),
			},
		})
		ctx.JSON(http.StatusBadRequest, response)
		return false
	}
	return true
}

func SenhaForte(fl validator.FieldLevel) bool {
	senha := fl.Field().String()

	if len(senha) < 8 {
		return false
	}

	temMinuscula, _ := regexp.MatchString(`[a-z]`, senha)
	if !temMinuscula {
		return false
	}

	temMaiuscula, _ := regexp.MatchString(`[A-Z]`, senha)
	if !temMaiuscula {
		return false
	}

	temNumero, _ := regexp.Match(`[0-9]`, []byte(senha))
	if !temNumero {
		return false
	}

	temSimbolo, _ := regexp.MatchString(`[!@#$%^&*()_+\-=[\]{};:'",.<>/?\\|]`, senha)
	if !temSimbolo {
		return false
	}

	return true
}

func DataValida(fl validator.FieldLevel) bool {
	data := fl.Field().String()
	match, _ := regexp.MatchString(`^\d{4}-\d{2}-\d{2}$`, data)
	if !match {
		return false
	}

	_, err := time.Parse("2006-01-02", data)
	if err != nil {
		return false
	}

	return true
}

func validatePF(c *models.Cliente) error {
	if c.TipoPessoa == "PF" {
		if c.CPF == "" {
			return errors.New("CPF é obrigatório para pessoa física")
		}
		return validaCPF(c.CPF)
	}
	return nil
}

func validatePJ(c *models.Cliente) error {
	if c.TipoPessoa == "PJ" {
		if c.CNPJ == "" {
			return errors.New("CNPJ é obrigatório para pessoa jurídica")
		}
		return validaCNPJ(c.CNPJ)
	}
	return nil
}

func validateRazaoSocial(c *models.Cliente) error {
	if c.TipoPessoa == "PJ" {
		if c.RazaoSocial == "" {
			return errors.New("Razão social é obrigatório para pessoa jurídica")
		}
	}
	return nil
}

func confirmaSenhasIguais(c *models.Cliente) error {
	if c.Senha != c.ConfirmarSenha {
		return errors.New("As duas senhas devem ser iguais")
	}
	return nil
}

func validaCPF(cpf string) error {
	if cpf == "" {
		return errors.New("CPF não pode ser vazio")
	}

	if len(cpf) != 11 {
		return errors.New("CPF deve ter exatamente 11 dígitos")
	}

	if _, err := strconv.Atoi(cpf); err != nil {
		return errors.New("CPF deve conter apenas números")
	}

	igual := true
	for i := 1; i < len(cpf); i++ {
		if cpf[i] != cpf[0] {
			igual = false
			break
		}
	}
	if igual {
		return errors.New("CPF não pode ter todos os dígitos iguais")
	}

	soma := 0
	for i := 0; i < 9; i++ {
		digito, _ := strconv.Atoi(string(cpf[i]))
		soma += digito * (10 - i)
	}

	resto := soma % 11
	if resto < 2 {
		if cpf[9] != '0' {
			return errors.New("CPF inválido")
		}
	} else {
		digitoVerificador := 11 - resto
		if cpf[9] != byte(digitoVerificador+'0') {
			return errors.New("CPF inválido")
		}
	}

	soma = 0
	for i := 0; i < 10; i++ {
		digito, _ := strconv.Atoi(string(cpf[i]))
		soma += digito * (11 - i)
	}

	resto = soma % 11
	if resto < 2 {
		if cpf[10] != '0' {
			return errors.New("CPF inválido")
		}
	} else {
		digitoVerificador := 11 - resto
		if cpf[10] != byte(digitoVerificador+'0') {
			return errors.New("CPF inválido")
		}
	}

	return nil
}

func validaCNPJ(cnpj string) error {
	if cnpj == "" {
		return errors.New("CNPJ não pode ser vazio")
	}

	if len(cnpj) != 14 {
		return errors.New("CNPJ deve ter exatamente 14 dígitos")
	}

	if _, err := strconv.Atoi(cnpj); err != nil {
		return errors.New("CNPJ deve conter apenas números")
	}

	igual := true
	for i := 1; i < len(cnpj); i++ {
		if cnpj[i] != cnpj[0] {
			igual = false
			break
		}
	}
	if igual {
		return errors.New("CNPJ não pode ter todos os dígitos iguais")
	}

	multiplicadores := []int{5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2}
	soma := 0
	for i := 0; i < 12; i++ {
		digito, _ := strconv.Atoi(string(cnpj[i]))
		soma += digito * multiplicadores[i]
	}

	resto := soma % 11
	if resto < 2 {
		if cnpj[12] != '0' {
			return errors.New("CNPJ inválido")
		}
	} else {
		digitoVerificador := 11 - resto
		if cnpj[12] != byte(digitoVerificador+'0') {
			return errors.New("CNPJ inválido")
		}
	}

	multiplicadores = []int{6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2}
	soma = 0
	for i := 0; i < 13; i++ {
		digito, _ := strconv.Atoi(string(cnpj[i]))
		soma += digito * multiplicadores[i]
	}

	resto = soma % 11
	if resto < 2 {
		if cnpj[13] != '0' {
			return errors.New("CNPJ inválido")
		}
	} else {
		digitoVerificador := 11 - resto
		if cnpj[13] != byte(digitoVerificador+'0') {
			return errors.New("CNPJ inválido")
		}
	}

	return nil
}

func LoginValido(login *models.Login, ctx *gin.Context) bool {
	if err := ctx.ShouldBindJSON(&login); err != nil {
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

	return true
}
