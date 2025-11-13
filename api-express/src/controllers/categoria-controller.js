import * as categoriaService from '../services/categoria-service.js';
import AppMessage from '../utils/app-message.js';

export default class CategoriaController {
    static async getAll(req, res) {
        const results = await categoriaService.getAll();
        return res.status(200).send(new AppMessage('Categorias obtidas com sucesso', 200, results));
    }
}