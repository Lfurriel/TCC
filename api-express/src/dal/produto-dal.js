import prisma from "../configs/prisma-client.js";
import AppMessage from "../utils/app-message.js";

export const getAll = async (skuOrSkuLst, fieldList, page = 1, pageSize = 10) => {
    const offset = (page - 1) * pageSize;
    const produtos = await prisma.produto.findMany({
        where: {
            sku: skuOrSkuLst
        },
        select: {
            ...fieldList,
            categoria: {
                select: {
                    id: true,
                    nome: true,
                }
            }
        },
        skip: offset,
        take: Number(pageSize),
    });

    await prisma.$disconnect();

    return produtos;
}

export const getRandom = async (fieldList, limite) => {
    const fields = fieldList
        ? Object.entries(fieldList)
            .filter(([, value]) => value)
            .map(([key]) => `"produtos"."${key}"`)
            .join(', ')
        : '*';

    const produtos = await prisma.$queryRawUnsafe(`
        SELECT ${fields}, categorias.id as categoria_id, categorias.nome as categoria_nome
        FROM "produtos"
        LEFT JOIN "categorias" ON "produtos"."idCategoria" = "categorias"."id"
        ORDER BY RANDOM()
        LIMIT ${Number(limite)};
    `);

    await prisma.$disconnect();

    return produtos;
};

export const getBySKUs = async (skus, fieldList) => {
    const produtos = await prisma.produto.findMany({
        where: {
            sku: {
                in: skus,
            },
        },
        select: fieldList ?? undefined,
    });

    await prisma.$disconnect();

    return produtos;
}

export const getByNome = async (nome, fieldList, page = 1, pageSize = 10) => {
    const offset = (page - 1) * pageSize;

    const produtos = await prisma.produto.findMany({
        where: {
            nome: {
                contains: nome,
            }
        },
        select: {
            ...fieldList,
            categoria: {
                select: {
                    id: true,
                    nome: true,
                }
            }
        },
        skip: offset,
        take: Number(pageSize),
    });

    await prisma.$disconnect();

    return produtos;
};

export const getByCategoria = async (idCategoria, fieldList, page = 1, pageSize = 10) => {
    const offset = (page - 1) * pageSize;

    const produtos = await prisma.produto.findMany({
        where: {
            idCategoria,
        },
        select: {
            ...fieldList,
            categoria: {
                select: {
                    id: true,
                    nome: true,
                }
            }
        },
        skip: offset,
        take: Number(pageSize),
    });

    await prisma.$disconnect();

    return produtos;
}

export const getBySKU = async (sku) => {
    const produto = await prisma.produto.findFirst({
        where: {
            sku: {
                equals: sku,
                mode: 'insensitive',
            },
        },
    });

    if (!produto) {
        throw new AppMessage('Produto não encontrado', 404)
    }

    await prisma.$disconnect();

    return produto;
}

export const updateEstoqueEVendas = async (sku, quantidadeEstoque, quantidadeVendas) => {
    const produto = await prisma.produto.findFirst({
        where: {
            sku,
        },
        select: {
            estoque: true,
        }
    });

    if (!produto) {
        throw new AppMessage('Produto não encontrado', 404)
    }

    await prisma.$queryRaw`UPDATE "produtos" SET "estoque"="estoque" + (${quantidadeEstoque}), "qtdvendas"="qtdvendas" + (${quantidadeVendas}) WHERE "sku" = ${sku}`;
};

export const getAllOfertas = async (fieldList, page, pageSize) => {
    const offset = (page - 1) * pageSize;
    const produtos = await prisma.produto.findMany({
        where: {
            pctoferta: {
                gt: 0,
            }
        },
        orderBy: {
            pctoferta: 'desc',
        },
        select: {
            ...fieldList,
            categoria: {
                select: {
                    id: true,
                    nome: true,
                }
            }
        },
        skip: offset,
        take: Number(pageSize)
    });

    await prisma.$disconnect();

    return produtos;
}

export const getAllDestaques = async (fieldList, page = 1, pageSize = 10) => {
    const offset = (page - 1) * pageSize;
    const produtos = await prisma.produto.findMany({
        orderBy: {
            pctoferta: 'desc',
        },
        select: {
            ...fieldList,
            categoria: {
                select: {
                    id: true,
                    nome: true,
                }
            }
        },
        skip: offset,
        take: Number(pageSize),
    });

    await prisma.$disconnect();

    return produtos;
}
