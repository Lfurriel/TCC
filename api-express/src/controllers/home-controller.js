import * as homeService from '../services/home-service.js';
import AppMessage from '../utils/app-message.js';

export default class HomeController {
    static async getAmazonHome(req, res) {
        const results = await homeService.getAmazonHome();
        return res.status(200).send(new AppMessage('Home obtida com sucesso', 200, results));
    }

    static async getShopeeHome(req, res) {
        const results = await homeService.getShopeeHome();
        return res.status(200).send(new AppMessage('Home obtida com sucesso', 200, results));
    }
}