# Análise de Desempenho de APIs RESTful para E-commerce: Comparação entre Actix, Express.js e Gin

Repositório contendo a implementação de três APIs RESTful equivalentes em diferentes frameworks (Actix, Express.js e Gin) para análise comparativa de desempenho em cenários de e-commerce de alta demanda.

## Sobre o projeto

Este projeto apresenta uma análise comparativa sistemática de desempenho entre três frameworks web modernos:

- Actix (Rust) - Framework de alta performance baseado no padrão Actor Model
- Express.js (Node.js) - Framework minimalista e amplamente adotado
- Gin (Go) - Framework otimizado para velocidade e eficiência

As APIs simulam duas plataformas reais de e-commerce com características distintas (Amazon e Shopee), diferenciadas principalmente pelo volume de dados retornados nas requisições.

## Objetivos
Análise comparativa de desempenho entre web frameworks (Actix Web, Express.js e Gin). 

Mais especificamente:
- Simular três APIs, baseadas em duas plataformas de e-commerce (Amazon e Shopee);
- Analisar o desempenho dessas APIs em quatro testes.

## Arquitetura
Todas as APIs seguem arquitetura RESTful padronizada, garantindo comparação justa:
- Endpoints equivalentes: Mesma estrutura de rotas e funcionalidades
- Banco de dados compartilhado: PostgreSQL único para todas as implementações
- Estrutura de dados idêntica: Formato JSON padronizado nas respostas
- Containerização: Docker para ambiente isolado e reprodutível

| Método |    Grupo   |              Caminho             |            Descrição           |
|--------|:-----------|:---------------------------------|:-------------------------------|
| GET    | Categorias | `/categorias`                    | Retorna categorias de produtos |
| POST   | Clientes   | `/clientes/login`                | Autentica cliente              |
| POST   | Clientes   | `/clientes`                      | Cria novo cliente              |
| GET    | Home       | `/home/amazon`                   | Página inicial Amazon          |
| GET    | Home       | `/home/shopee`                   | Página inicial Shopee          |
| POST   | Pedidos    | `/pedidos`                       | Cria novo pedido               |
| GET    | Produtos   | `/produtos/`                     | Lista todos os produtos        |
| GET    | Produtos   | `/produtos/:sku`                 | Retorna produto específico     |
| GET    | Produtos   | `/produtos/categoria/:categoria` | Produtos por categoria         |
| GET    | Produtos   | `/produtos/ofertas`              | Produtos em oferta             |
| GET    | Produtos   | `/produtos/destaques`            | Produtos em destaque           |
| GET    | Produtos   | `/produtos/nome`                 | Busca produto por nome         |

## Testes de performance
Os testes foram desenvolvidos utilizando k6 e estão organizados em quatro categorias:

### 1. Teste de Carga (Load Test)
Avalia comportamento sob condições normais de operação.
- **Duração**: 22 minutos
- **Patamares**: 500 VUs (10 min) → 750 VUs (10 min)

### 2. Teste de Estresse (Stress Test)
Examina limites operacionais sob condições progressivamente intensas.
- **Duração**: 24 minutos
- **Patamares**:  800 → 1.000 → 1.200 → 1.400 VUs (5 min cada)

### 3. Teste de Pico (Spike Test)
Simula surtos súbitos de tráfego extremo (flash sales, promoções).
- **Duração**: 12 minutos 30 segundos
- **Patamares**:  100 VUs → 2.500 VUs (em 10 segundos, mantidos por 10 min)

### 4. Teste de Ruptura (Breakpoint Test)
Identifica ponto máximo de capacidade antes do colapso.
- **Duração**: 25 minutos
- **Patamares**:  500 → 7.000 VUs (progressão linear +375 VUs/min)

## Principais resultados
Resumo Comparativo
|  Framework | Throughput Médio |   Latência p95  | Taxa de Falhas | Uso de CPU |
|:-----------|:-----------------|----------------:|---------------:|-----------:|
| Gin        | 252-387 req/s    | 868-1.311 ms    | 0-0,34%        | 10-35%     |
| Actix      | 185-239 req/s    | 492-2.794 ms    | 0,01-0,03%     | 20-68%     |
| Express.js | 96-131 req/s     | 7.897-15.091 ms | 3,56-7,26%     | 18-30%     |

## Metodologia
Ambiente de Execução

Especificações dos Contêineres:
- Sistema Operacional: Alpine Linux
- vCPUs: 4.0
- Memória RAM: 4GB

Máquina Hospedeira:

- Processador: AMD Ryzen 7 5700X3D (8 núcleos, 16 threads)
- RAM: 32GB
- SO: Windows 11
- Virtualização: Docker

Métricas Avaliadas

- Throughput: Requisições processadas por segundo
- Latência: Tempo de resposta (mediana, p90, p95)
- Uso de CPU: Porcentagem de utilização do processador
- Uso de Memória: Consumo de RAM durante os testes
- Taxa de Falhas: Porcentagem de requisições que falharam

### Tecnologias utilizadas
| Tecnologia |  Versão |            Principais Dependências            |
|:-----------|:--------|:----------------------------------------------|
| Rust       | 1.83.0  | actix-web 4.10.2, tokio 1.44.2, diesel 2.2.10 |
| Node.js    | 18.19.0 | express 4.21.2, prisma 6.4.1, bcryptjs 3.0.2  |
| Go         | 1.24.1  | gin-gonic/gin 1.10.0, gorm 1.25.12            |

Ferramentas

- Banco de Dados: PostgreSQL 15.03
- Testes de Performance: k6
- Containerização: Docker
- Dataset: Amazon Brazil Products 2023 (1.3M produtos)

## Limitações e trabalhos futuros
Limitações Identificadas

1. Ambiente de teste local (não em nuvem)
2. Dataset simulado (não dados reais de produção)

Sugestões para Extensão

1. Comparação com FastAPI (Python), Spring Boot (Java), ASP.NET Core (C#)
2. Análise de diferentes SGBDs (MySQL, MongoDB)
3. Testes de longa duração (soak tests 24-72h)
4. Testes em ambientes de nuvem (AWS, GCP, Azure)

## Autores
- Gabriel Scarano de Oliveira
- Lucas Furriel Rodrigues

**Orientadora**: Prof.ª Dra. Adriana Barbosa Santo 

**Instituição**: UNESP - Instituto de Biociências, Letras e Ciências Exatas
Curso: Bacharelado em Ciência da Computação
**Ano**: 2025