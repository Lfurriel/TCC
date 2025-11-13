import * as produtoDal from '../dal/produto-dal.js';

export const getAll = async (page, pageSize) => {
    return produtoDal.getAll(undefined, {
        sku: true,
        nome: true,
        foto: true,
        pctoferta: true,
        preco: true,
        descricao: true,
        estoque: true,
    }, page, pageSize);
};

export const getBySKU = async (sku) => {
    return produtoDal.getBySKU(sku, {
        sku: true,
        nome: true,
        foto: true,
        pctoferta: true,
        preco: true,
        descricao: true,
        estoque: true,
    });
};

export const getByCategoria = async (idCategoria, page, pageSize) => {
    return produtoDal.getByCategoria(idCategoria, {
        sku: true,
        nome: true,
        foto: true,
        pctoferta: true,
        preco: true,
    }, page, pageSize);
}

export const getAllOfertas = async (page, pageSize) => {
    return produtoDal.getAllOfertas({
        sku: true,
        nome: true,
        foto: true,
        pctoferta: true,
        preco: true,
        descricao: true,
        estoque: true,
    }, page, pageSize);
}

export const getAllDestaques = async (page, pageSize) => {
    return produtoDal.getAllDestaques({
        sku: true,
        nome: true,
        foto: true,
        pctoferta: true,
        preco: true,
        descricao: true,
        estoque: true,
    }, page, pageSize);
}

export const getByNome = async (nome, page, pageSize) => {
    return produtoDal.getByNome(nome, {
        sku: true,
        nome: true,
        foto: true,
        pctoferta: true,
        preco: true,
        descricao: true,
        estoque: true,
    }, page, pageSize);
};
