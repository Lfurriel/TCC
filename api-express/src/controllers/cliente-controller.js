import { createClienteSchema, loginSchema } from "../validations/cliente-validations.js";
import * as clienteService from '../services/cliente-service.js'
import AppMessage from '../utils/app-message.js'

export default class ClienteController {
    static async create(req, res) {
        const payload = createClienteSchema.parse(req.body);

        const result = await clienteService.create(payload);
        return res.status(201).send(new AppMessage('Cliente criado com sucesso', 201, result));
    }

    static async login(req, res) {
        const payload = loginSchema.parse(req.body);

        const result = await clienteService.login(payload);
        return res.status(200).send(new AppMessage('Login realizado com sucesso', 200, result))
    }
}