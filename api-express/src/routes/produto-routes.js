import { Router } from "express";
import ProdutoController from "../controllers/produto-controller.js";

const produtoRoutes = Router();

produtoRoutes.get('/categoria/:id', ProdutoController.getByCategoria);
produtoRoutes.get('/ofertas', ProdutoController.getAllOfertas);
produtoRoutes.get('/destaques', ProdutoController.getAllDestaques);
produtoRoutes.get('/nome', ProdutoController.getByNome);
produtoRoutes.get('/:sku', ProdutoController.getBySKU);
produtoRoutes.get('/', ProdutoController.getAll);

export default produtoRoutes;