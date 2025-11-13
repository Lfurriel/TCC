package models

import "time"

type Produto struct {
	SKU         string    `json:"sku,omitempty" gorm:"primaryKey;column:sku;type:text;not null"`
	Codigo      int       `json:"codigo,omitempty" gorm:"type:int;column:codigo;not null"`
	IdCategoria string    `json:"idCategoria,omitempty" gorm:"type:text;column:idCategoria;not null"`
	Nome        string    `json:"nome,omitempty" gorm:"size:120;column:nome;not null"`
	Descricao   *string   `json:"descricao,omitempty" gorm:"type:text;column:descricao"`
	Foto        *string   `json:"foto,omitempty" gorm:"type:text;column:foto"`
	Preco       float64   `json:"preco,omitempty" gorm:"type:decimal(11,2);column:preco;not null"`
	Estoque     float64   `json:"estoque,omitempty" gorm:"type:decimal(13,4);column:estoque;not null"`
	PctOferta   float64   `json:"pctOferta,omitempty" gorm:"type:decimal(11,2);column:pctoferta;not null"`
	QtdVendas   float64   `json:"qtdVendas,omitempty" gorm:"type:integer;column:qtdvendas;not null"`
	CreatedAt   time.Time `json:"-" gorm:"autoCreateTime;column:createdAt;not null"`
	UpdatedAt   time.Time `json:"-" gorm:"autoUpdateTime;column:updatedAt;not null"`

	// Chave estrangeira para Categoria
	Categoria Categoria `json:"categoria" gorm:"foreignKey:IdCategoria;references:ID"`
}

func (Produto) TableName() string {
	return "produtos"
}
