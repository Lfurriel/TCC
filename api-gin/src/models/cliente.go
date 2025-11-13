package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type Cliente struct {
	ID             string    `json:"id,omitempty" gorm:"primaryKey;column:id;type:varchar(36);not null"`
	TipoPessoa     string    `json:"tipoPessoa,omitempty" gorm:"type:char(2);column:tipoPessoa;not null" binding:"required,oneof=PF PJ"`
	CPF            string    `json:"cpf,omitempty" gorm:"type:varchar(11);column:cpf"`
	CNPJ           string    `json:"cnpj,omitempty" gorm:"type:varchar(14);column:cnpj"`
	Nome           string    `json:"nome,omitempty" gorm:"type:varchar(60);column:nome;not null" binding:"required,min=1,max=60"`
	IE             string    `json:"ie,omitempty" gorm:"type:varchar(14);column:ie" binding:"omitempty,min=7,max=14"`
	RazaoSocial    string    `json:"razaoSocial,omitempty" gorm:"type:varchar(60);column:razaoSocial"`
	DataNascimento string    `json:"dataNascimento,omitempty" gorm:"type:date;column:dataNascimento;not null" binding:"required,data_valida"`
	Sexo           string    `json:"sexo,omitempty" gorm:"type:char(1);column:sexo;not null" binding:"required,oneof=M F N"`
	Email          string    `json:"email,omitempty" gorm:"type:text;column:email;not null" binding:"required,email"`
	Telefone       string    `json:"telefone,omitempty" gorm:"type:varchar(14);column:telefone;not null" binding:"required,min=1,max=14"`
	Senha          string    `json:"senha,omitempty" gorm:"type:text;column:senha;not null" binding:"required,senha_forte"`
	ConfirmarSenha string    `json:"confirmarSenha,omitempty" gorm:"-" binding:"required"`
	CreatedAt      time.Time `json:"createdAt" gorm:"autoCreateTime;column:createdAt;not null"`
	UpdatedAt      time.Time `json:"updatedAt" gorm:"autoUpdateTime;column:updatedAt;not null"`
}

func (Cliente) TableName() string {
	return "clientes"
}

func (c *Cliente) BeforeCreate(tx *gorm.DB) (err error) {
	uuidStr := uuid.New().String()
	c.ID = uuidStr
	return
}

type Login struct {
	Email string `json:"email" binding:"required,email"`
	Senha string `json:"senha" binding:"required"`
}
