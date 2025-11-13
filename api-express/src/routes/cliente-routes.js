import { Router } from "express";
import ClienteController from "../controllers/cliente-controller.js";
import PedidoController from "../controllers/pedido-controller.js";
import isAuthenticated from "../middlewares/is-authenticated.js";

const clienteRoutes = Router();

clienteRoutes.post('/', ClienteController.create);
clienteRoutes.post('/login', ClienteController.login);

export default clienteRoutes;