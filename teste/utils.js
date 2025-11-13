import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';
import { randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

export function gerarCPF() {
    const n = [];
    for (let i = 0; i < 9; i++) {
        n.push(randomIntBetween(0, 9));
    }

    // Dígito 1
    let d1 = n.map((v, i) => v * (10 - i)).reduce((a, b) => a + b, 0);
    d1 = 11 - (d1 % 11);
    if (d1 >= 10) d1 = 0;

    // Dígito 2
    let d2 = [...n, d1].map((v, i) => v * (11 - i)).reduce((a, b) => a + b, 0);
    d2 = 11 - (d2 % 11);
    if (d2 >= 10) d2 = 0;

    return n.join('') + d1 + d2;
}

export function gerarCNPJ() {
    const n = [];
    for (let i = 0; i < 8; i++) {
        n.push(randomIntBetween(0, 9));
    }

    // Fixo para simplificação: filial "0001"
    n.push(0, 0, 0, 1);

    const calcDV = (base, pesos) => {
        const soma = base.reduce((sum, num, idx) => sum + num * pesos[idx], 0);
        let dv = soma % 11;
        return dv < 2 ? 0 : 11 - dv;
    };

    const pesos1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    const d1 = calcDV(n, pesos1);
    const pesos2 = [6].concat(pesos1);
    const d2 = calcDV([...n, d1], pesos2);

    return n.join('') + d1 + d2;
}


export function gerarEmail() {
    return `usuario_${uuidv4().substring(0, 8)}@teste.com`;
}

export function getRandomElement(array) {
    return array[randomIntBetween(0, array.length - 1)];
}
