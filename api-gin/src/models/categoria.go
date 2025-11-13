package models

import (
	"github.com/google/uuid"
	"gorm.io/gorm"
	"time"
)

type Categoria struct {
	ID        string    `json:"id" gorm:"primaryKey;column:id;type:varchar(36);not null"`
	Nome      string    `json:"nome" gorm:"type:varchar(30);column:nome;not null"`
	CreatedAt time.Time `json:"-" gorm:"autoCreateTime;column:createdAt;not null"`
	UpdatedAt time.Time `json:"-" gorm:"autoUpdateTime;column:updatedAt;not null"`
}

func (Categoria) TableName() string {
	return "categorias"
}

func (c *Categoria) BeforeCreate(tx *gorm.DB) (err error) {
	c.ID = uuid.New().String()
	return
}
