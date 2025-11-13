import { Router } from "express";
import categoriaRoutes from "./categoria-routes.js";
import produtoRoutes from "./produto-routes.js";
import clienteRoutes from "./cliente-routes.js";
import pedidoRoutes from "./pedido-routes.js";
import homeRoutes from "./home-routes.js";

const routes = Router();

routes.use('/categorias', categoriaRoutes);
routes.use('/clientes', clienteRoutes);
routes.use('/pedidos', pedidoRoutes);
routes.use('/produtos', produtoRoutes);
routes.use('/home', homeRoutes);

export default routes;