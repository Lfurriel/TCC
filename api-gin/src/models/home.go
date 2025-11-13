package models

type HomeAmazon struct {
	Ofertas    []Produto   `json:"ofertas"`    // 4 (sku, foto, pctOferta)
	Categorias []Categoria `json:"categorias"` // ALL (id, nome)
	Produtos   []Produto   `json:"produtos"`   // 20 em 7 categorias (sku, foto, categoria)
}

type HomeShopee struct {
	Ofertas    []Produto   `json:"ofertas"`    // 15 ofertas (sku, foto, pctOferta, preco)
	Produtos   []Produto   `json:"produtos"`   // 36 (sku, nome, foto, pctOferta, preco)
	Categorias []Categoria `json:"categorias"` // ALL (id, nome)
	Destaques  []Produto   `json:"destaques"`  // 18 (sku, nome, qtdVendas, foto)
}
