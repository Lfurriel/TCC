package dal

import (
	"errors"
	"gorm.io/gorm"
	"net/http"
	"tcc-api-gin/src/configs"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func CreateCliente(cliente models.Cliente) (*models.Cliente, *utils.RestErr) {
	var clienteExiste *models.Cliente

	var err error
	if cliente.TipoPessoa == "PF" {
		err = configs.DB.Where("email = ?", cliente.Email).
			Or("cpf = ?", cliente.CPF).
			First(&clienteExiste).Error
	} else {
		err = configs.DB.Where("email = ?", cliente.Email).
			Or("cnpj = ?", cliente.CNPJ).
			First(&clienteExiste).Error
	}

	if err != nil && !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao verificar se o cliente já existe", err)
	}

	if clienteExiste.ID != "" {
		msg := map[bool]string{
			cliente.Email == clienteExiste.Email:                                                 "Cliente com email já cadastrado",
			cliente.CNPJ != "" && clienteExiste.CNPJ != "" && cliente.CNPJ == clienteExiste.CNPJ: "Cliente com CNPJ já cadastrado",
			cliente.CPF != "" && clienteExiste.CPF != "" && cliente.CPF == clienteExiste.CPF:     "Cliente com CPF já cadastrado",
		}[true]
		if msg == "" {
			msg = "Cliente já existe"
		}

		return nil, utils.NewRestErr(http.StatusBadRequest, msg, nil)
	}

	result := configs.DB.Create(&cliente)
	if result.Error != nil {
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao criar cliente", err)
	}

	return &cliente, nil
}

func GetClienteByEmail(email string) (*models.Cliente, *utils.RestErr) {
	var clienteExiste models.Cliente
	err := configs.DB.Where("email = ?", email).First(&clienteExiste).Error

	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, utils.NewRestErr(http.StatusNotFound, "Cliente não encontrado", err)
		}
		return nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao buscar cliente", err)
	}

	return &clienteExiste, nil
}
