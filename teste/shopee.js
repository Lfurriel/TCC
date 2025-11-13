import http from 'k6/http';
import { check, sleep } from 'k6';
import { Counter } from 'k6/metrics';
import { randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';
import { spikeOptions, BASE_URL } from './config.js';
import { gerarEmail, gerarCPF, gerarCNPJ, getRandomElement } from './utils.js';

export { spikeOptions as options };

const req_success = new Counter('req_success');
const req_failed = new Counter('req_failed');

const SEARCH_TERMS = [
    "televisão", "alexa", "monitor"
];

const USER_CREDENTIALS = [
    { email: "l.furriel35@gmail.com", senha: "Senha@123" }
];

// Função para verificar resposta e atualizar métricas
function checkResponse(response, context = '') {
    check(response, {
        'Status 2xx (success)': (r) => r.status >= 200 && r.status < 300,
        'Response time less than 30s': (r) => r.timings.duration < 30000,
    });

    const isSuccess = response.status >= 200 && response.status < 300;

    if (isSuccess) {
        req_success.add(1);
        return true;
    } else {
        console.error(`Falha na requisição ${context}: Status ${response.status}`);
        req_failed.add(1);
        return false;
    }
}

export default function () {
    let visitedProducts = [];
    let homeData = null;

    try {
        // 1. Acessa a home page
        homeData = accessHomePage();
        if (!homeData) {
            return;
        }

        // 2. Realiza navegação por produtos
        const totalFlows = randomIntBetween(3, 5);
        for (let i = 0; i < totalFlows; i++) {
            const productsFound = executeProductNavigation(homeData);
            visitedProducts.push(...productsFound);

            // 15% dos usuários saem após primeira navegação (bounce)
            if (i === 0 && Math.random() < 0.15) {
                return;
            }

            sleep(randomIntBetween(3, 8));
        }

        // 10% dos usuários saem antes de fazer login/cadastro
        if (Math.random() < 0.10) {
            return;
        }

        // 3. Autentica usuário
        const token = authenticateUser();
        if (!token) {
            return;
        }

        // 4. Realiza pedido
        if (visitedProducts.length > 0) {
            createOrder(visitedProducts, token);
            sleep(randomIntBetween(5, 15));
        } else {
            console.error("Usuário não encontrou produtos para comprar");
        }

    } catch (error) {
        req_failed.add(1);
        console.error(`Erro não tratado no fluxo: ${error.message}`);
    }
}

function accessHomePage() {
    const homeRes = http.get(`${BASE_URL}/home/shopee`);

    // Verifica resposta e atualiza métricas
    if (!checkResponse(homeRes, 'Home Page Shopee'))
        return null;

    return homeRes.json().data;
}

function executeProductNavigation(homeData) {
    const option = randomIntBetween(1, 5);

    try {
        switch (option) {
            case 1:
                return navigateOfferPages();
            case 2:
                return navigateCategoryPages(homeData);
            case 3:
                return navigateHighlightPages();
            case 4:
                return accessDirectProduct(homeData);
            case 5:
                return searchProductsByName();
        }
    } catch (error) {
        req_failed.add(1);
        console.error(`Erro na navegação opção ${option}: ${error.message}`);
        return [];
    }
}

function navigateOfferPages() {
    const totalPages = randomIntBetween(2, 5);
    let foundProducts = [];

    for (let page = 1; page <= totalPages; page++) {
        const ofertasRes = http.get(`${BASE_URL}/produtos/ofertas?page=${page}&pageSize=20`);

        // Verifica resposta e atualiza métricas
        if (!checkResponse(ofertasRes, `Ofertas página ${page}`)) {
            continue;
        }

        try {
            const products = ofertasRes.json().data;
            if (Array.isArray(products) && products.length > 0) {
                foundProducts = products;
            }
        } catch (e) {
            console.error(`Erro ao parsear ofertas página ${page}`);
        }

        sleep(randomIntBetween(1, 2));
    }

    return selectAndViewProduct(foundProducts);
}

function navigateCategoryPages(homeData) {
    if (!homeData.categorias || homeData.categorias.length === 0) {
        return [];
    }

    const randomCategory = getRandomElement(homeData.categorias);
    const totalPages = randomIntBetween(2, 5);
    let foundProducts = [];

    for (let page = 1; page <= totalPages; page++) {
        const categoriaRes = http.get(`${BASE_URL}/produtos/categoria/${randomCategory.id}?page=${page}&pageSize=20`);

        // Verifica resposta e atualiza métricas
        if (!checkResponse(categoriaRes, `Categoria ${randomCategory.id} página ${page}`)) {
            continue;
        }

        try {
            const products = categoriaRes.json().data;
            if (Array.isArray(products) && products.length > 0) {
                foundProducts = products;
            }
        } catch (e) {
            console.error(`Erro ao parsear categoria página ${page}: ${e.message}`);
        }

        sleep(randomIntBetween(1, 2));
    }

    return selectAndViewProduct(foundProducts);
}

function navigateHighlightPages() {
    const totalPages = randomIntBetween(2, 5);
    let foundProducts = [];

    for (let page = 1; page <= totalPages; page++) {
        const destaquesRes = http.get(`${BASE_URL}/produtos/destaques?page=${page}&pageSize=20`);

        // Verifica resposta e atualiza métricas
        if (!checkResponse(destaquesRes, `Destaques página ${page}`)) {
            continue;
        }

        try {
            const products = destaquesRes.json().data;
            if (Array.isArray(products) && products.length > 0) {
                foundProducts = products;
            }
        } catch (e) {
            console.error(`Erro ao parsear destaques página ${page}: ${e.message}`);
        }

        sleep(randomIntBetween(1, 2));
    }

    return selectAndViewProduct(foundProducts);
}

function accessDirectProduct(homeData) {
    if (!homeData.ofertas || homeData.ofertas.length === 0) {
        return [];
    }

    const randomOferta = getRandomElement(homeData.ofertas);
    return viewProduct(randomOferta.sku) ? [randomOferta.sku] : [];
}

function searchProductsByName() {
    const nomeProduto = getRandomElement(SEARCH_TERMS);
    const nomeRes = http.get(`${BASE_URL}/produtos/nome?name=${encodeURIComponent(nomeProduto)}`);

    // Verifica resposta e atualiza métricas
    if (!checkResponse(nomeRes, `Busca por "${nomeProduto}"`)) {
        return [];
    }

    try {
        const produtos = nomeRes.json().data;
        if (!Array.isArray(produtos) || produtos.length === 0) {
            return [];
        }

        return selectAndViewProduct(produtos);
    } catch (e) {
        console.error(`Erro ao parsear busca por nome: ${e.message}`);
        return [];
    }
}

function selectAndViewProduct(products) {
    if (!Array.isArray(products) || products.length === 0) {
        return [];
    }

    const selectedProduct = getRandomElement(products);
    const success = viewProduct(selectedProduct.sku);

    return success ? [selectedProduct.sku] : [];
}

function viewProduct(sku) {
    const produtoRes = http.get(`${BASE_URL}/produtos/${sku}`);

    if (!checkResponse(produtoRes, `Visualizar produto ${sku}`)) {
        return false;
    }

    try {
        const productData = produtoRes.json().data;
        return productData && productData.sku;
    } catch (e) {
        console.error(`Erro ao parsear produto ${sku}: ${e.message}`);
        return false;
    }
}

function authenticateUser() {
    const optionLogin = randomIntBetween(1, 2);

    if (optionLogin === 1) {
        return performLogin();
    } else {
        return performRegistration();
    }
}

function performLogin() {
    const credentials = getRandomElement(USER_CREDENTIALS);

    const payloadLogin = JSON.stringify({
        email: credentials.email,
        senha: credentials.senha
    });

    const loginRes = http.post(`${BASE_URL}/clientes/login`, payloadLogin, {
        headers: {
            'Content-Type': 'application/json',
        }
    });

    // Verifica resposta e atualiza métricas
    if (!checkResponse(loginRes, 'Login')) {
        return null;
    }

    try {
        const loginData = loginRes.json();
        const token = (loginData.data && loginData.data.token) || loginData.token;

        if (!token) {
            console.error("Token não encontrado na resposta de login");
            req_failed.add(1);
            return null;
        }

        return token;
    } catch (e) {
        console.error(`Login - Erro ao parsear JSON: ${e.message}`);
        req_failed.add(1);
        return null;
    }
}

function performRegistration() {
    const tipoPessoa = Math.random() < 0.5 ? 'PF' : 'PJ';
    const emailAleatorio = gerarEmail();

    let payload;
    if (tipoPessoa === 'PF') {
        payload = {
            tipoPessoa: 'PF',
            cpf: gerarCPF(),
            nome: `Teste PF ${randomIntBetween(1000, 9999)}`,
            dataNascimento: '2000-01-01',
            sexo: Math.random() < 0.5 ? 'M' : 'F',
            email: emailAleatorio,
            telefone: `119${randomIntBetween(10000000, 99999999)}`,
            senha: 'Senha@123',
            confirmarSenha: 'Senha@123'
        };
    } else {
        payload = {
            tipoPessoa: 'PJ',
            cnpj: gerarCNPJ(),
            razaoSocial: `Empresa Teste ${randomIntBetween(1000, 9999)} Ltda`,
            nome: `Empresa Teste ${randomIntBetween(1000, 9999)}`,
            dataNascimento: '2000-01-01',
            ie: `${randomIntBetween(100000000, 999999999)}`,
            sexo: 'M',
            email: emailAleatorio,
            telefone: `119${randomIntBetween(10000000, 99999999)}`,
            senha: 'Senha@123',
            confirmarSenha: 'Senha@123'
        };
    }

    const cadastroRes = http.post(`${BASE_URL}/clientes`, JSON.stringify(payload), {
        headers: { 'Content-Type': 'application/json' }
    });

    // Verifica resposta e atualiza métricas
    if (!checkResponse(cadastroRes, 'Cadastro')) {
        return null;
    }

    try {
        const cadastroData = cadastroRes.json();
        const token = (cadastroData.data && cadastroData.data.token) || cadastroData.token;

        if (!token) {
            console.error("Token não encontrado na resposta de cadastro");
            console.error("Resposta de cadastro:", JSON.stringify(cadastroRes.json(), null, 2));
            req_failed.add(1);
            return null;
        }

        return token;
    } catch (e) {
        console.error(`Cadastro - Erro ao parsear JSON: ${e.message}`);
        req_failed.add(1);
        return null;
    }
}

function createOrder(visitedProducts, token) {
    const uniqueProducts = [...new Set(visitedProducts)];
    const maxProdutos = Math.min(uniqueProducts.length, 5);

    const produtosPedido = uniqueProducts
        .slice(0, maxProdutos)
        .map(sku => ({
            skuProduto: sku,
            quantidade: randomIntBetween(1, 3)
        }));

    const formasPagamento = ['B', 'P', 'D', 'C'];
    const formaPagamento = getRandomElement(formasPagamento);

    let porcentagemDesconto = 0;
    if (formaPagamento === 'B') {
        porcentagemDesconto = 0.1;
    } else if (formaPagamento === 'P') {
        porcentagemDesconto = 0.05;
    }

    const pedidoPayload = {
        produtos: produtosPedido,
        enderecoEntrega: {
            nomeRemetente: `Cliente Teste ${randomIntBetween(1000, 9999)}`,
            cep: "15057439",
            logradouro: "Rua Irmã Heloísa Helena de Campos Melo",
            numero: `${randomIntBetween(100, 999)}`,
            complemento: `Apto ${randomIntBetween(1, 300)}`,
            bairro: "Residencial Santa Filomena",
            codigoIbgeCidade: "3549805",
            codigoIbgeUF: "31"
        },
        pagamento: {
            formaPagamento: formaPagamento,
            numeroParcelas: randomIntBetween(1, formaPagamento === 'C' ? 12 : 3),
            porcentagemDesconto: porcentagemDesconto
        }
    };

    const pedidoRes = http.post(`${BASE_URL}/pedidos`, JSON.stringify(pedidoPayload), {
        headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${token}`
        }
    });

    // Verifica resposta e atualiza métricas
    return checkResponse(pedidoRes, 'Criar pedido');
}