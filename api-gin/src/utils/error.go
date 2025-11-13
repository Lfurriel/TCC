package utils

import (
	"fmt"
	"github.com/gin-gonic/gin"
)

type RestErr struct {
	Code int
	Msg  string
	Err  error
}

func NewRestErr(code int, msg string, err error) *RestErr {
	return &RestErr{Code: code, Msg: msg, Err: err}
}

func RespondRestErr(err *RestErr, ctx *gin.Context) {
	if err != nil && err.Err != nil {
		fmt.Println(err.Err.Error())
	}

	ctx.JSON(err.Code, NewAppMessage(
		err.Msg,
		err.Code,
		nil,
	))

	ctx.Abort()
}
