import { createPedidoSchema } from "../validations/pedido-validations.js";
import * as pedidoService from '../services/pedido-service.js'
import AppMessage from '../utils/app-message.js'

export default class PedidoController {
    static async create(req, res) {
        const payload = createPedidoSchema.parse(req.body);
        payload.idCliente = req.cliente.id;

        const result = await pedidoService.create(payload);
        return res.status(201).send(new AppMessage('Pedido criado com sucesso', 201, result));
    }
}