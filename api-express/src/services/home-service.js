import * as produtosDal from '../dal/produto-dal.js'
import * as categoriasDal from '../dal/categoria-dal.js'

export const getAmazonHome = async () => {
    const home = {};

    const [ofertas, categorias] = await Promise.all([
        produtosDal.getAllOfertas(
            {
                sku: true,
                foto: true,
                pctoferta: true,
                idCategoria: true,
            },
            1,
            4
        ),
        categoriasDal.getAll()
    ]);

    home.ofertas = ofertas;

    const categoriasCopy = [...categorias];
    if (categoriasCopy.length > 7) {
        for (let i = categoriasCopy.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [categoriasCopy[i], categoriasCopy[j]] = [categoriasCopy[j], categoriasCopy[i]];
        }
        home.categorias = categoriasCopy.slice(0, 7);
    } else {
        home.categorias = categoriasCopy;
    }

    const produtoPromises = home.categorias.map(categoria =>
        produtosDal.getByCategoria(
            categoria.id,
            {
                sku: true,
                nome: true,
                foto: true,
                preco: true,
                idCategoria: true,
            },
            1,
            20
        )
    );

    const produtosResults = await Promise.all(produtoPromises);
    home.produtos = produtosResults.filter(produtos => produtos).flat();

    return home;
};

export const getShopeeHome = async () => {
    const home = {};

    const [ofertas, categorias, produtos, destaques] = await Promise.all([
        produtosDal.getAllOfertas(
            {
                sku: true,
                foto: true,
                pctoferta: true,
                preco: true,
                idCategoria: true,
            },
            1,
            15
        ),
        categoriasDal.getAll(),
        produtosDal.getRandom(
            {
                sku: true,
                nome: true,
                foto: true,
                pctoferta: true,
                preco: true,
                idCategoria: true,
            },
            36
        ),
        produtosDal.getAllDestaques(
            {
                sku: true,
                nome: true,
                pctoferta: true,
                idCategoria: true,
                qtdvendas: true,
            },
            1,
            15
        )
    ]);

    home.ofertas = ofertas;
    home.categorias = categorias;
    home.produtos = produtos;
    home.destaques = destaques;

    return home;
};