/*
  Warnings:

  - You are about to drop the column `valorTotal` on the `pedidos` table. All the data in the column will be lost.
  - Added the required column `valorBruto` to the `pedidos` table without a default value. This is not possible if the table is not empty.
  - Added the required column `valorLiquido` to the `pedidos` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "pedidos" DROP COLUMN "valorTotal",
ADD COLUMN     "valorBruto" DECIMAL(11,2) NOT NULL,
ADD COLUMN     "valorLiquido" DECIMAL(11,2) NOT NULL;
