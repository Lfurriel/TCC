/*
  Warnings:

  - Added the required column `updatedAt` to the `clientes` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "clientes" ADD COLUMN     "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "updatedAt" TIMESTAMP(3) NOT NULL;

-- CreateTable
CREATE TABLE "pedidos" (
    "id" TEXT NOT NULL,
    "idCliente" TEXT NOT NULL,
    "valorTotal" DECIMAL(11,2) NOT NULL,
    "valorFrete" DECIMAL(11,2) NOT NULL,
    "valorDesconto" DECIMAL(11,2) NOT NULL,
    "status" CHAR(1) NOT NULL,
    "dataEntrega" DATE NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pedidos_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "produtosPedido" (
    "id" TEXT NOT NULL,
    "idPedido" TEXT NOT NULL,
    "skuProduto" TEXT NOT NULL,
    "quantidade" DECIMAL(11,2) NOT NULL,
    "valorUnitario" DECIMAL(11,2) NOT NULL,
    "valorFrete" DECIMAL(11,2) NOT NULL,
    "valorDesconto" DECIMAL(11,2) NOT NULL,
    "valorTotal" DECIMAL(11,2) NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "produtosPedido_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "pedidos" ADD CONSTRAINT "pedidos_idCliente_fkey" FOREIGN KEY ("idCliente") REFERENCES "clientes"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "produtosPedido" ADD CONSTRAINT "produtosPedido_idPedido_fkey" FOREIGN KEY ("idPedido") REFERENCES "pedidos"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "produtosPedido" ADD CONSTRAINT "produtosPedido_skuProduto_fkey" FOREIGN KEY ("skuProduto") REFERENCES "produtos"("sku") ON DELETE RESTRICT ON UPDATE CASCADE;
