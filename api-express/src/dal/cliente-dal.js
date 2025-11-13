import prisma from "../configs/prisma-client.js";
import AppMessage from "../utils/app-message.js";

export const create = async (payload) => {
    const clienteExists = await prisma.cliente.findFirst({
        where: {
            OR: [
                {
                    email: {
                        equals: payload.email,
                        mode: 'insensitive',
                    },
                },
                {
                    cpf: {
                        equals: payload.cpf,
                        mode: 'insensitive',
                    },
                },
                {
                    cnpj: {
                        equals: payload.cnpj,
                        mode: 'insensitive',
                    },
                }
            ]
        },
        select: {
            email: true,
            cnpj: true,
            cpf: true,
        }
    });

    if (clienteExists) {
        const msg = payload.email == clienteExists.email ? 'Cliente com email já cadastrado'
                : payload.cnpj == clienteExists.cnpj && payload.cnpj != undefined ? 'Cliente com CNPJ já cadastrado'
                : payload.cpf == clienteExists.cpf && payload.cpf != undefined ? 'Cliente com CPF já cadastrado'
                : '';

        throw new AppMessage(msg, 400);
    }

    const cliente = await prisma.cliente.create({
        data: payload,
    });

    await prisma.$disconnect();

    return cliente;
};

export const getByEmail = async (email) => {
    const cliente = await prisma.cliente.findFirst({
        where: {
            email,
        },
    });

    if (!cliente) {
        throw new AppMessage('Cliente não encontrado', 404);
    }

    await prisma.$disconnect();

    return cliente;
};