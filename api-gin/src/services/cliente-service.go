package services

import (
	"github.com/golang-jwt/jwt"
	"net/http"
	"os"
	"tcc-api-gin/src/dal"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
	"time"
)

func CreateCliente(cliente models.Cliente) (string, *models.Cliente, *utils.RestErr) {
	var err error
	cliente.Senha, err = utils.HashPassword(cliente.Senha)
	if err != nil {
		return "", nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao criptografar senha", err)
	}
	cliente.ConfirmarSenha = ""
	clienteCriado, restErr := dal.CreateCliente(cliente)
	if restErr != nil {
		return "", nil, restErr
	}
	token, err := generateToken(clienteCriado)
	if err != nil {
		return "", nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao gerar token", err)
	}
	return token, clienteCriado, nil
}

func Login(login models.Login) (string, *models.Cliente, *utils.RestErr) {
	cliente, restErr := dal.GetClienteByEmail(login.Email)
	if restErr != nil {
		return "", nil, restErr
	}

	if !utils.ComparePassword(login.Senha, cliente.Senha) {
		return "", nil, utils.NewRestErr(http.StatusUnauthorized, "Senha incorreta", nil)
	}

	cliente.Senha = ""
	cliente.CPF = ""
	cliente.CNPJ = ""

	token, err := generateToken(cliente)
	if err != nil {
		return "", nil, utils.NewRestErr(http.StatusInternalServerError, "Erro ao gerar token", err)
	}
	return token, cliente, nil
}

func generateToken(cliente *models.Cliente) (string, error) {
	secret := os.Getenv("JWT_TOKEN")

	claims := jwt.MapClaims{
		"cliente": cliente,                               // Dados do cliente
		"exp":     time.Now().Add(time.Hour * 24).Unix(), // Expiração do token (1 dia)
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString([]byte(secret))
}
