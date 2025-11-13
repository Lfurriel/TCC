/*
  Warnings:

  - You are about to drop the column `valorTotal` on the `produtosPedido` table. All the data in the column will be lost.
  - Added the required column `valorBruto` to the `produtosPedido` table without a default value. This is not possible if the table is not empty.
  - Added the required column `valorLiquido` to the `produtosPedido` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "produtosPedido" DROP COLUMN "valorTotal",
ADD COLUMN     "valorBruto" DECIMAL(11,2) NOT NULL,
ADD COLUMN     "valorLiquido" DECIMAL(11,2) NOT NULL;
