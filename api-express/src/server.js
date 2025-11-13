import 'express-async-errors';
import 'reflect-metadata';
import express from 'express';
import cors from 'cors';
import AppMessage from './utils/app-message.js';
import routes from './routes/routes.js';
import { ZodError } from 'zod';
import prisma from './configs/prisma-client.js';

const app = express();

app.use(cors());
app.use(express.json());
app.use(routes);

app.use((error, request, response, next) => {
    if (error instanceof AppMessage) {
        if (error.statusCode > 299) {
            return response.status(error.statusCode).json({
                statusCode: error.statusCode,
                status: 'error',
                message: error.message,
            });
        }
    } else if (error instanceof ZodError) {
        response.status(400).json({
            status: "error",
            message: "Erro de Validação",
            errors: error.errors,
        });
    } else {
        console.log(error)
        return response.status(500).json({
            status: 'error',
            message: 'Erro interno',
        });
    }
});

app.listen(3333, async () => {
    console.log(`Server listening at http://localhost:3333/`)

    // for (let i = 1; i <= 10; i++) {
    //     const produtos = []
    //     for (let j = 1; j <= 10; j++) {
    //         produtos.push({
    //             sku: `${i}${j}`,
    //             codigo: j,
    //             nome: `Produto ${j} da Categoria ${i}`,
    //             descricao: `Descrição ${j}`,
    //             preco: 15 * j,
    //             estoque: 10 * j,
    //         });
    //     }

    //     const categoria = {
    //         nome: `Categoria ${i}`
    //     }

    //     await prisma.categoria.create({
    //         data: {
    //             ...categoria,
    //             produtos: {
    //                 createMany: {
    //                     data: produtos,
    //                 },
    //             },
    //         },
    //     })
    // }
    // const categorias = [
    //     {
    //         nome: 'Teste 1',
    //     },
    //     {
    //         nome: 'Teste 2',
    //     },
    //     {
    //         nome: '',
    //     },
    //     {
    //         nome: '',
    //     },
    //     {
    //         nome: '',
    //     },
    //     {
    //         nome: '',
    //     },
    // ];
})