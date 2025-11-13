/*
  Warnings:

  - You are about to drop the column `valorDesconto` on the `pagamentos` table. All the data in the column will be lost.
  - You are about to drop the column `valorJuros` on the `pagamentos` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "pagamentos" DROP COLUMN "valorDesconto",
DROP COLUMN "valorJuros";
