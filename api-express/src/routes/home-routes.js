import { Router } from "express";
import HomeController from "../controllers/home-controller.js";

const homeRoutes = Router();

homeRoutes.get('/amazon', HomeController.getAmazonHome);
homeRoutes.get('/shopee', HomeController.getShopeeHome);

export default homeRoutes;