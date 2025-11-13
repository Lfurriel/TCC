import * as produtoService from '../services/produto-service.js';
import AppMessage from '../utils/app-message.js';

export default class ProdutoController {
    static async getAll(req, res) {
        const results = await produtoService.getAll();
        return res.status(200).send(new AppMessage('Produtos obtidos com sucesso', 200, results))
    }

    static async getBySKU(req, res) {
        const sku = req.params.sku;
        const result = await produtoService.getBySKU(sku);
        return res.status(200).send(new AppMessage('Produto obtido com sucesso', 200, result))
    }

    static async getByCategoria(req, res) {
        const idCategoria = req.params.idCategoria;
        const { page, pageSize } = req.query;
        const results = await produtoService.getByCategoria(idCategoria, page ?? 1, pageSize ?? 10);

        return res.status(200).send(new AppMessage('Produtos obtidos com sucesso', 200, results))
    }

    static async getAllOfertas(req, res) {
        const { page, pageSize } = req.query;
        const results = await produtoService.getAllOfertas(page ?? 1, pageSize ?? 10);

        return res.status(200).send(new AppMessage('Produtos obtidos com sucesso', 200, results))
    }

    static async getAllDestaques(req, res) {
        const { page, pageSize } = req.query;
        const results = await produtoService.getAllDestaques(page ?? 1, pageSize ?? 10);

        return res.status(200).send(new AppMessage('Produtos obtidos com sucesso', 200, results))
    }

    static async getByNome(req, res) {
        const { page, pageSize, name } = req.query;
        const results = await produtoService.getByNome(name, page ?? 1, pageSize ?? 10);

        return res.status(200).send(new AppMessage('Produtos obtidos com sucesso', 200, results))
    }
}