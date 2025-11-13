package configs

import (
	"github.com/gin-gonic/gin/binding"
	"github.com/go-playground/validator/v10"
	"tcc-api-gin/src/validations"
)

func BindingValidator() {
	if v, ok := binding.Validator.Engine().(*validator.Validate); ok {
		v.RegisterValidation("senha_forte", validations.SenhaForte)
		v.RegisterValidation("data_valida", validations.DataValida)
	}
}
