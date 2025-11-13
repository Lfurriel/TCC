-- CreateTable
CREATE TABLE "enderecosEntrega" (
    "id" TEXT NOT NULL,
    "idPedido" TEXT NOT NULL,
    "nomeRemetente" VARCHAR(120) NOT NULL,
    "cep" VARCHAR(8) NOT NULL,
    "logradouro" VARCHAR(60) NOT NULL,
    "numero" VARCHAR(60) NOT NULL,
    "complemento" VARCHAR(60),
    "bairro" VARCHAR(60) NOT NULL,
    "codigoIbgeCidade" VARCHAR(7) NOT NULL,
    "codigoIbgeUF" VARCHAR(2) NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "enderecosEntrega_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pagamentos" (
    "id" TEXT NOT NULL,
    "idPedido" TEXT NOT NULL,
    "formaPagamento" VARCHAR(1) NOT NULL,
    "numeroParcelas" SMALLINT NOT NULL,
    "valorParcela" DECIMAL(11,2) NOT NULL,
    "valorDesconto" DECIMAL(11,2) NOT NULL,
    "valorJuros" DECIMAL(11,2) NOT NULL,
    "valorTotal" DECIMAL(11,2) NOT NULL,
    "boleto" VARCHAR(48),
    "pix" TEXT,
    "tid" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pagamentos_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "enderecosEntrega" ADD CONSTRAINT "enderecosEntrega_idPedido_fkey" FOREIGN KEY ("idPedido") REFERENCES "pedidos"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pagamentos" ADD CONSTRAINT "pagamentos_idPedido_fkey" FOREIGN KEY ("idPedido") REFERENCES "pedidos"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
