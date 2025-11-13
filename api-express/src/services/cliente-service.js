import * as clienteDal from '../dal/cliente-dal.js';
import AppMessage from '../utils/app-message.js';
import { comparePasswords, hashPassword } from '../utils/hash-password.js';
import jwt from 'jsonwebtoken';
import authConfig from '../configs/auth.js';

export const create = async (payload) => {
    const senhaSemHash = payload.senha;
    payload.senha = await hashPassword(payload.senha);
    payload.confirmarSenha = undefined;
    const { senha, ...clienteSemSenha } = await clienteDal.create(payload);

    const tokenPayload = await login({ email: clienteSemSenha.email, senha: senhaSemHash });

    return {
        ...clienteSemSenha,
        token: tokenPayload.token
    }
};

export const login = async (payload) => {
    const cliente = await clienteDal.getByEmail(payload.email);

    if (!(await comparePasswords(payload.senha, cliente.senha))) {
        throw new AppMessage('Senha incorreta', 401);
    }

    cliente.senha = undefined;
    cliente.cpf = undefined;
    cliente.cnpj = undefined;

    const token = jwt.sign(
        { cliente: JSON.stringify(cliente) },
        authConfig.jwt.secret,
        { expiresIn: authConfig.jwt.expiresIn }
    );

    return {
        cliente,
        token,
    }
};