import * as categoria from '../dal/categoria-dal.js';

export const getAll = async () => {
    return categoria.getAll();
};