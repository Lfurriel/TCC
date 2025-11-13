import prisma from "../configs/prisma-client.js";
import AppError from "../utils/app-message.js";

export const getAll = async () => {
    const categorias = await prisma.categoria.findMany();

    await prisma.$disconnect();

    return categorias;
}
