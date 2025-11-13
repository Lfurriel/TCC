import { Router } from "express";
import PedidoController from "../controllers/pedido-controller.js";
import isAuthenticated from "../middlewares/is-authenticated.js";

const pedidoRoutes = Router();

pedidoRoutes.post('/', isAuthenticated, PedidoController.create);

export default pedidoRoutes;