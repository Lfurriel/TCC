package services

import (
	"math/rand/v2"
	"sync"
	"tcc-api-gin/src/dal"
	"tcc-api-gin/src/models"
	"tcc-api-gin/src/utils"
)

func GetAmazonHome() (*models.HomeAmazon, *utils.RestErr) {
	var home models.HomeAmazon
	var wg sync.WaitGroup
	var mu sync.Mutex
	var firstError *utils.RestErr

	errChan := make(chan *utils.RestErr, 2)

	wg.Add(1)
	go func() {
		defer wg.Done()
		ofertas, restErr := dal.GetAllOfertas("sku, foto, pctOferta, \"idCategoria\"", 1, 4)
		if restErr != nil {
			errChan <- restErr
			return
		}
		mu.Lock()
		home.Ofertas = *ofertas
		mu.Unlock()
	}()

	// Goroutine para buscar categorias
	wg.Add(1)
	go func() {
		defer wg.Done()
		categorias, restErr := dal.GetAllCategoria()
		if restErr != nil {
			errChan <- restErr
			return
		}

		allCategorias := *categorias
		if len(allCategorias) > 7 {
			rand.Shuffle(len(allCategorias), func(i, j int) {
				allCategorias[i], allCategorias[j] = allCategorias[j], allCategorias[i]
			})
		}

		mu.Lock()
		home.Categorias = allCategorias
		mu.Unlock()
	}()
	wg.Wait()
	close(errChan)

	for err := range errChan {
		if firstError == nil {
			firstError = err
		}
	}
	if firstError != nil {
		return nil, firstError
	}

	var produtos []models.Produto
	var produtosMu sync.Mutex
	var produtosWg sync.WaitGroup
	produtosErrChan := make(chan *utils.RestErr, len(home.Categorias))

	categoriasParaProdutos := home.Categorias
	if len(home.Categorias) > 7 {
		categoriasParaProdutos = home.Categorias[:7]
	}

	for _, categoria := range categoriasParaProdutos {
		produtosWg.Add(1)
		go func(cat models.Categoria) {
			defer produtosWg.Done()
			produtosCategoria, restErr := dal.GetAllProdutosByCategoria(cat.ID, "sku, produtos.nome, foto, preco, \"idCategoria\"", 1, 20)
			if restErr != nil {
				produtosErrChan <- restErr
				return
			}
			if produtosCategoria != nil {
				produtosMu.Lock()
				produtos = append(produtos, *produtosCategoria...)
				produtosMu.Unlock()
			}
		}(categoria)
	}

	produtosWg.Wait()
	close(produtosErrChan)

	for err := range produtosErrChan {
		if firstError == nil {
			firstError = err
		}
	}
	if firstError != nil {
		return nil, firstError
	}

	home.Produtos = produtos

	return &home, nil
}

func GetShoppeHome() (*models.HomeShopee, *utils.RestErr) {
	var home models.HomeShopee
	var wg sync.WaitGroup
	var mu sync.Mutex
	var firstError *utils.RestErr

	// Canal para capturar erros
	errChan := make(chan *utils.RestErr, 4)

	wg.Add(1)
	go func() {
		defer wg.Done()
		ofertas, restErr := dal.GetAllOfertas("sku, foto, pctoferta, preco, \"idCategoria\"", 1, 15)
		if restErr != nil {
			errChan <- restErr
			return
		}
		mu.Lock()
		home.Ofertas = *ofertas
		mu.Unlock()
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		categorias, restErr := dal.GetAllCategoria()
		if restErr != nil {
			errChan <- restErr
			return
		}
		mu.Lock()
		home.Categorias = *categorias
		mu.Unlock()
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		produtos, restErr := dal.GetRandomProdutos("sku, produtos.nome, foto, pctoferta, preco, \"idCategoria\"", 36)
		if restErr != nil {
			errChan <- restErr
			return
		}
		mu.Lock()
		home.Produtos = produtos
		mu.Unlock()
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		destaques, restErr := dal.GetAllDestaques("sku, produtos.nome, pctoferta, \"idCategoria\", qtdvendas", 1, 15)
		if restErr != nil {
			errChan <- restErr
			return
		}
		mu.Lock()
		home.Destaques = *destaques
		mu.Unlock()
	}()

	wg.Wait()
	close(errChan)

	for err := range errChan {
		if firstError == nil {
			firstError = err
		}
	}
	if firstError != nil {
		return nil, firstError
	}

	return &home, nil
}
