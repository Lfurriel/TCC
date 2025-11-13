import jwt from 'jsonwebtoken';
import authConfig from '../configs/auth.js';
import { request } from 'express';
import AppMessage from '../utils/app-message.js';

export default function isAuthenticated(req, res, next) {
  console.log("oi");
    const authHeader = req.headers.authorization;
    const { verify } = jwt;
    
    if (!authHeader) {
        throw new AppMessage('Token de autenticação não informado.', 401);
    }
    
    const [, token] = authHeader.split(' ');

    try {
        const decodedToken = verify(token, authConfig.jwt.secret);
        
        const { cliente } = decodedToken;
        
        req.cliente = {
            id: JSON.parse(cliente).id,
        };
        
    return next();
  } catch {
    console.log("oi2");
    throw new AppMessage('Invalid JWT Token');
  }
}