import prisma from '../configs/prisma-client.js';
import AppMessage from '../utils/app-message.js';

export const create = async (payload) => {
    const { enderecoEntrega, pagamento, produtos, ...resto } = payload;

    // Validar valores numéricos
    const validateNumber = (value, defaultValue = 0) => {
        if (value === undefined || value === null || Number.isNaN(value)) {
            return defaultValue;
        }
        return Number(value);
    };

    const pedidoData = {
        idCliente: resto.idCliente,
        valorBruto: validateNumber(resto.valorBruto),
        valorDesconto: validateNumber(resto.valorDesconto),
        dataEntrega: resto.dataEntrega,
        valorFrete: validateNumber(resto.valorFrete),
        valorLiquido: validateNumber(resto.valorLiquido),
        status: resto.status
    };

    const pagamentoData = {
        formaPagamento: pagamento.formaPagamento,
        numeroParcelas: validateNumber(pagamento.numeroParcelas, 1),
        valorTotal: validateNumber(pagamento.valorTotal),
        valorParcela: validateNumber(pagamento.valorParcela)
    };
    
    if (pagamento.porcentagemDesconto !== undefined && pagamento.porcentagemDesconto !== null) {
        pagamentoData.porcentagemDesconto = validateNumber(pagamento.porcentagemDesconto);
    }

    const produtosData = produtos.map(produto => ({
        skuProduto: produto.skuProduto,
        quantidade: validateNumber(produto.quantidade, 1),
        valorUnitario: validateNumber(produto.valorUnitario),
        valorBruto: validateNumber(produto.valorBruto),
        valorDesconto: validateNumber(produto.valorDesconto),
        valorLiquido: validateNumber(produto.valorLiquido),
        valorFrete: validateNumber(produto.valorFrete)
    }));

    const pedido = await prisma.pedido.create({
        data: {
            ...pedidoData,
            enderecosEntrega: {
                create: { ...enderecoEntrega }
            },
            pagamentos: {
                create: pagamentoData
            },
            produtos: {
                createMany: {
                    data: produtosData
                }
            },
        },
        include: {
            enderecosEntrega: true,
            pagamentos: true,
            produtos: true,
        }
    });

    return pedido;
}
