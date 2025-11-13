import { Router } from "express";
import CategoriaController from "../controllers/categoria-controller.js";

const categoriaRoutes = Router();

categoriaRoutes.get('/', CategoriaController.getAll);

export default categoriaRoutes;