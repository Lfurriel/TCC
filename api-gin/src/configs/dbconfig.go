package configs

import (
	"fmt"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/schema"
	"log"
	"os"
	"tcc-api-gin/src/models"
	"time"
)

var DB *gorm.DB

func ConnectToDb() {
	var errConnection error

	host := os.Getenv("DATABASE_HOST")
	user := os.Getenv("DATABASE_USER")
	password := os.Getenv("DATABASE_PASS")
	name := os.Getenv("DATABASE_NAME")
	port := os.Getenv("DATABASE_PORT")
	sslRequired := os.Getenv("DATABASE_SSL")
	ssl := "disable"
	if sslRequired == "true" {
		ssl = "require"
	}

	dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s sslmode=%s TimeZone=UTC-3", host, user, password, name, port, ssl)

	DB, errConnection = gorm.Open(postgres.Open(dsn), &gorm.Config{
		NamingStrategy: schema.NamingStrategy{
			SingularTable: true,
			NoLowerCase:   true,
		},
	})

	if errConnection != nil {
		panic("Erro ao conectar com banco de dados")
	}

	sqlDB, err := DB.DB()
	if err != nil {
		panic("Erro ao obter instância SQL do GORM")
	}

	sqlDB.SetMaxOpenConns(50)           // Máximo de conexões abertas
	sqlDB.SetMaxIdleConns(10)           // Máximo de conexões inativas
	sqlDB.SetConnMaxLifetime(time.Hour) // Tempo máximo de vida de uma conexão

	migrate()

	fmt.Println("Banco de dados conectado")
}

func migrate() {
	err := DB.AutoMigrate(
		&models.Categoria{},
		&models.Cliente{},
		&models.EnderecoEntrega{},
		&models.Pagamento{},
		&models.Pedido{},
		&models.Produto{},
		&models.ProdutoPedido{},
	)

	if err != nil {
		log.Fatalf("Erro ao realizar AutoMigrate: %v", err)
	}

	fmt.Println("Migrações aplicadas com sucesso.")
}
