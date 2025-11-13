import * as pedidoDal from '../dal/pedido-dal.js';
import * as produtoDal from '../dal/produto-dal.js'
import AppMessage from '../utils/app-message.js'
import { mapFrete } from '../utils/tabela-frete.js';

export const create = async (payload) => {
    const skuProdutos = payload.produtos.map(p => p.skuProduto);
    payload.valorBruto = 0;
    payload.valorDesconto = 0;

    if (!skuProdutos.length) {
        throw new AppMessage('Nenhum produto informado', 400);
    }

    
    const lstProdutos = await produtoDal.getAll({
        in: skuProdutos
    }, {
        sku: true,
    })
    
    const mapProdutos = new Map();
    for (const produto of lstProdutos) {
        mapProdutos.set(produto.sku, produto);
    }
    
    if (skuProdutos.length != lstProdutos.lenght) {
        for (const sku of skuProdutos) {
            if (!mapProdutos.has(sku)) {
                throw new AppMessage(`Produto com SKU igual a "${sku}" não existe`, 400);
            }
        }
    }

    const valorFrete = mapFrete.get(Number(payload.enderecoEntrega.codigoIbgeUF));

    for (const produto of payload.produtos) {
        if (mapProdutos.get(produto.skuProduto).estoque < produto.quantidade) {
            throw new AppMessage(`Produto com SKU igual a "${produto.skuProduto}" está com estoque em falta`, 400);
        }
        
        produto.valorUnitario = mapProdutos.get(produto.skuProduto).preco;
        produto.valorBruto = produto.quantidade * produto.valorUnitario;
        produto.valorDesconto = payload.pagamento.porcentagemDesconto ? Math.round((produto.valorBruto * payload.pagamento.porcentagemDesconto) * 100) / 100 : 0;
        produto.valorLiquido = produto.valorBruto - produto.valorDesconto;
        produto.valorFrete = Math.round((valorFrete/(payload.produtos.length)) * 100) / 100;

        payload.valorBruto += produto.valorBruto;
        payload.valorDesconto += produto.valorDesconto;
    }

    // data de entrega
    let data = new Date();
    data.setDate(data.getDate() + 3);

    const dataISO = data.toISOString();
    payload.dataEntrega = dataISO;
    // fim data de entrega

    payload.valorFrete = valorFrete;
    payload.valorLiquido = payload.valorBruto - payload.valorDesconto + payload.valorFrete;

    payload.pagamento.porcentagemDesconto = undefined;
    payload.pagamento.valorTotal = payload.valorLiquido;
    payload.pagamento.valorParcela = Math.round((payload.pagamento.valorTotal/payload.pagamento.numeroParcelas) * 100) / 100;
    
    payload.enderecoEntrega.codigoIbgeUF = payload.enderecoEntrega.codigoIbgeUF.toString();
    payload.enderecoEntrega.codigoIbgeCidade = payload.enderecoEntrega.codigoIbgeCidade.toString();
    payload.status = 'P';

    const pedido = await pedidoDal.create(payload);

    for (const produto of pedido.produtos) {
        await produtoDal.updateEstoqueEVendas(produto.skuProduto, produto.quantidade * -1, produto.quantidade);
    }

    return pedido;
};
