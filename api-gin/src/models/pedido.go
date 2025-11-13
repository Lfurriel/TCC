package models

import (
	"github.com/google/uuid"
	"gorm.io/gorm"
	"time"
)

type Pedido struct {
	ID            string    `json:"id,omitempty" gorm:"primaryKey;column:id;type:varchar(36);not null" binding:"-"`
	IdCliente     string    `json:"idCliente,omitempty" gorm:"column:idCliente;not null" binding:"-"`
	ValorBruto    float64   `json:"valorBruto,omitempty" gorm:"type:decimal(11,2);column:valorBruto;not null" binding:"-"`
	ValorFrete    float64   `json:"valorFrete,omitempty" gorm:"type:decimal(11,2);column:valorFrete;not null" binding:"-"`
	ValorDesconto float64   `json:"valorDesconto,omitempty" gorm:"type:decimal(11,2);column:valorDesconto;not null" binding:"-"`
	ValorLiquido  float64   `json:"valorLiquido,omitempty" gorm:"type:decimal(11,2);column:valorLiquido;not null" binding:"-"`
	Status        string    `json:"status,omitempty" gorm:"type:char(1);column:status;not null" binding:"-"`
	DataEntrega   time.Time `json:"dataEntrega,omitempty" gorm:"type:date;column:dataEntrega;not null" binding:"-"`
	CreatedAt     time.Time `json:"createdAt" gorm:"autoCreateTime;column:createdAt;not null" binding:"-"`
	UpdatedAt     time.Time `json:"updatedAt" gorm:"autoUpdateTime;column:updatedAt;not null" binding:"-"`

	// Relações
	Cliente   Cliente         `json:"cliente,-" gorm:"foreignKey:IdCliente;constraint:OnUpdate:CASCADE,OnDelete:SET NULL;" binding:"-"`
	Produtos  []ProdutoPedido `json:"produtos" gorm:"foreignKey:IdPedido" binding:"required"`
	Endereco  EnderecoEntrega `json:"enderecoEntrega" gorm:"foreignKey:IdPedido" binding:"required"`
	Pagamento Pagamento       `json:"pagamento" gorm:"foreignKey:IdPedido" binding:"required"`
}

func (Pedido) TableName() string {
	return "pedidos"
}

func (p *Pedido) BeforeCreate(tx *gorm.DB) (err error) {
	p.ID = uuid.New().String()
	return
}

type ProdutoPedido struct {
	ID            string    `json:"id" gorm:"primaryKey;column:id;type:varchar(36);not null"`
	IdPedido      string    `json:"idPedido" gorm:"column:idPedido;not null"`
	SKUProduto    string    `json:"skuProduto" gorm:"column:skuProduto;not null" binding:"required"`
	Quantidade    float64   `json:"quantidade" gorm:"type:decimal(11,2);column:quantidade;not null" binding:"required"`
	ValorUnitario float64   `json:"valorUnitario" gorm:"type:decimal(11,2);column:valorUnitario;not null"`
	ValorBruto    float64   `json:"valorBruto" gorm:"type:decimal(11,2);column:valorBruto;not null"`
	ValorFrete    float64   `json:"valorFrete" gorm:"type:decimal(11,2);column:valorFrete;not null"`
	ValorDesconto float64   `json:"valorDesconto" gorm:"type:decimal(11,2);column:valorDesconto;not null"`
	ValorLiquido  float64   `json:"valorLiquido" gorm:"type:decimal(11,2);column:valorLiquido;not null"`
	CreatedAt     time.Time `json:"createdAt" gorm:"autoCreateTime;column:createdAt;not null"`
	UpdatedAt     time.Time `json:"updatedAt" gorm:"autoUpdateTime;column:updatedAt;not null"`

	// Relacionamentos
	Pedido  Pedido  `json:"-" gorm:"foreignKey:IdPedido;constraint:OnUpdate:CASCADE,OnDelete:CASCADE;"`
	Produto Produto `json:"-" gorm:"foreignKey:SKUProduto;references:SKU;constraint:OnUpdate:CASCADE,OnDelete:CASCADE;"`
}

func (ProdutoPedido) TableName() string {
	return "produtosPedido"
}

func (p *ProdutoPedido) BeforeCreate(tx *gorm.DB) (err error) {
	p.ID = uuid.New().String()
	return
}

type EnderecoEntrega struct {
	ID               string    `json:"id" gorm:"primaryKey;column:id;type:varchar(36);not null"`
	IdPedido         string    `json:"idPedido" gorm:"column:idPedido;not null"`
	NomeRemetente    string    `json:"nomeRemetente" gorm:"type:varchar(120);column:nomeRemetente;not null" binding:"required"`
	CEP              string    `json:"cep" gorm:"type:char(8);column:cep;not null" binding:"required,numeric,len=8"`
	Logradouro       string    `json:"logradouro" gorm:"type:varchar(60);column:logradouro;not null" binding:"required,min=1,max=60"`
	Numero           string    `json:"numero" gorm:"type:varchar(10);column:numero;not null" binding:"required,min=1,max=60"`
	Complemento      *string   `json:"complemento,omitempty" gorm:"type:varchar(60);column:complemento" binding:"omitempty,min=1,max=60"`
	Bairro           string    `json:"bairro" gorm:"type:varchar(60);column:bairro;not null" binding:"required,min=1,max=60"`
	CodigoIbgeCidade string    `json:"codigoIbgeCidade" gorm:"type:char(7);column:codigoIbgeCidade;not null" binding:"required,numeric,min=7,max=8"`
	CodigoIbgeUF     string    `json:"codigoIbgeUF" gorm:"type:char(2);column:codigoIbgeUF;not null" binding:"required,number,len=2"`
	CreatedAt        time.Time `json:"createdAt" gorm:"autoCreateTime;column:createdAt;not null"`
	UpdatedAt        time.Time `json:"updatedAt" gorm:"autoUpdateTime;column:updatedAt;not null"`

	// Relacionamento com Pedido
	Pedido *Pedido `json:"-" gorm:"foreignKey:IdPedido;constraint:OnUpdate:CASCADE,OnDelete:CASCADE;"`
}

func (EnderecoEntrega) TableName() string {
	return "enderecosEntrega"
}

func (e *EnderecoEntrega) BeforeCreate(tx *gorm.DB) (err error) {
	e.ID = uuid.New().String()
	return
}

type Pagamento struct {
	ID                  string    `json:"id" gorm:"primaryKey;column:id;type:text;not null"`
	IdPedido            string    `json:"idPedido" gorm:"column:idPedido;not null"`
	FormaPagamento      string    `json:"formaPagamento" gorm:"type:varchar(1);column:formaPagamento;not null" binding:"required,oneof=B P D C"`
	NumeroParcelas      int       `json:"numeroParcelas" gorm:"type:smallint;column:numeroParcelas;not null" binding:"required,number,gte=1,lte=12"`
	PorcentagemDesconto *float64  `json:"porcentagemDesconto" gorm:"-" binding:"omitempty,number,gte=0,lte=100"`
	ValorParcela        float64   `json:"valorParcela" gorm:"type:decimal(11,2);column:valorParcela;not null"`
	ValorTotal          float64   `json:"valorTotal" gorm:"type:decimal(11,2);column:valorTotal;not null"`
	Boleto              *string   `json:"boleto,omitempty" gorm:"type:varchar(48);column:boleto"`
	Pix                 *string   `json:"pix,omitempty" gorm:"type:text;column:pix"`
	TID                 *string   `json:"tid,omitempty" gorm:"type:text;column:tid"`
	CreatedAt           time.Time `json:"createdAt" gorm:"autoCreateTime;column:createdAt;not null"`
	UpdatedAt           time.Time `json:"updatedAt" gorm:"autoUpdateTime;column:updatedAt;not null"`

	// Relação com Pedido
	Pedido *Pedido `json:"-" gorm:"foreignKey:IdPedido;constraint:OnUpdate:CASCADE,OnDelete:CASCADE;"`
}

func (Pagamento) TableName() string {
	return "pagamentos"
}

func (p *Pagamento) BeforeCreate(tx *gorm.DB) (err error) {
	p.ID = uuid.New().String()
	return
}
