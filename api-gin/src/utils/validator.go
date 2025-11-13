package utils

import (
	"github.com/go-playground/validator/v10"
	"strings"
)

type ValidationError struct {
	Expected string `json:"expected,omitempty"`
	Path     string `json:"path,omitempty"`
	Message  string `json:"message,omitempty"`
}

func MapValidationError(err validator.FieldError) ValidationError {
	var validationError ValidationError
	validationError.Message = "Required"
	validationError.Path = err.Field()

	switch err.Tag() {
	case "required":
		validationError.Expected = "valor obrigatório"
	case "email":
		validationError.Expected = "email válido"
		validationError.Message = "O campo deve conter um email válido."
	case "len":
		validationError.Expected = err.Param() + " caracteres"
		validationError.Message = "O campo deve ter no exatos " + err.Param() + " caracteres."
	case "min":
		validationError.Expected = "mínimo de " + err.Param() + " caracteres"
		validationError.Message = "O campo deve ter no mínimo " + err.Param() + " caracteres."
	case "max":
		validationError.Expected = "máximo de " + err.Param() + " caracteres"
		validationError.Message = "O campo deve ter no máximo " + err.Param() + " caracteres."
	case "gte":
		validationError.Expected = "número maior que " + err.Param()
		validationError.Message = "O número deve ser maior que " + err.Param()
	case "lte":
		validationError.Expected = "número menor que " + err.Param()
		validationError.Message = "O número deve ser menor que " + err.Param()
	case "number":
		validationError.Expected = "numero"
		validationError.Message = "O campo deve ser um numero"
	case "numeric":
		validationError.Expected = "numero"
		validationError.Message = "O campo deve ser uma string numérica"
	case "oneof":
		validationError.Expected = strings.ReplaceAll(err.Param(), " ", " | ")
		validationError.Message = "O valor deve ser um dos seguintes: " + validationError.Expected
	case "senha_forte":
		validationError.Expected = "mínimo de 8 caracteres, uma letra maiúscula, uma minúscula e um símbolo"
		validationError.Message = "A senha deve ter pelo menos 8 caracteres, incluindo uma letra maiúscula, uma minúscula, um número e um símbolo."
	case "data_valida":
		validationError.Expected = "Data de nascimento válida"
		validationError.Message = "A data de nascimento deve estar no padrão yyyy-MM-dd"
	}

	return validationError
}
