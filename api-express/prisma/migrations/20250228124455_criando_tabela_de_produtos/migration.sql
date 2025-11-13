-- CreateTable
CREATE TABLE "produtos" (
    "sku" TEXT NOT NULL,
    "codigo" INTEGER NOT NULL,
    "idCategoria" TEXT NOT NULL,
    "nome" VARCHAR(120) NOT NULL,
    "descricao" TEXT NOT NULL,
    "foto" TEXT NOT NULL,
    "preco" DECIMAL(11,2) NOT NULL,
    "estoque" DECIMAL(13,4) NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "produtos_pkey" PRIMARY KEY ("sku")
);

-- AddForeignKey
ALTER TABLE "produtos" ADD CONSTRAINT "produtos_idCategoria_fkey" FOREIGN KEY ("idCategoria") REFERENCES "categorias"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
