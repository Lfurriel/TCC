package middleware

import (
	"errors"
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt"
	"net/http"
	"os"
	"strings"
	"tcc-api-gin/src/utils"
)

func IsAuthenticated(ctx *gin.Context) {
	secret := os.Getenv("JWT_TOKEN")
	tokenValue := removeBearerPrefix(ctx.Request.Header.Get("Authorization"))

	token, err := jwt.Parse(tokenValue, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); ok {
			return []byte(secret), nil
		}
		return nil, errors.New("Token inválido")
	})

	if err != nil {
		restErr := utils.NewRestErr(http.StatusUnauthorized, "Token inválido", err)
		utils.RespondRestErr(restErr, ctx)
		return
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	clienteData, ok := claims["cliente"].(map[string]interface{})
	if !ok {
		restErr := utils.NewRestErr(http.StatusUnauthorized, "Formato inválido do token", nil)
		utils.RespondRestErr(restErr, ctx)
		return
	}

	clienteID, exists := clienteData["id"].(string)
	if !exists {
		restErr := utils.NewRestErr(http.StatusUnauthorized, "Token sem ID do cliente", nil)
		utils.RespondRestErr(restErr, ctx)
		return
	}

	ctx.Set("cliente", clienteID)
}

func removeBearerPrefix(token string) string {
	return strings.TrimPrefix(token, "Bearer ")
}
