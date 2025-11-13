/*
  Warnings:

  - You are about to alter the column `razaoSocial` on the `clientes` table. The data in that column could be lost. The data in that column will be cast from `VarChar(120)` to `VarChar(60)`.
  - You are about to alter the column `nome` on the `clientes` table. The data in that column could be lost. The data in that column will be cast from `VarChar(120)` to `VarChar(60)`.

*/
-- AlterTable
ALTER TABLE "clientes" ALTER COLUMN "razaoSocial" SET DATA TYPE VARCHAR(60),
ALTER COLUMN "nome" SET DATA TYPE VARCHAR(60);
