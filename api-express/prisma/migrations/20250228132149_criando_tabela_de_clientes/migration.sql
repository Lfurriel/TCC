-- CreateTable
CREATE TABLE "clientes" (
    "id" TEXT NOT NULL,
    "tipoPessoa" CHAR(2) NOT NULL,
    "cpf" VARCHAR(11),
    "cnpj" VARCHAR(14),
    "razaoSocial" VARCHAR(120) NOT NULL,
    "nome" VARCHAR(120) NOT NULL,
    "dataNascimento" DATE NOT NULL,
    "ie" VARCHAR(14),
    "sexo" CHAR(1) NOT NULL,
    "email" TEXT NOT NULL,
    "telefone" VARCHAR(14) NOT NULL,
    "senha" TEXT NOT NULL,

    CONSTRAINT "clientes_pkey" PRIMARY KEY ("id")
);
