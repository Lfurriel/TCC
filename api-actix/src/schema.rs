// @generated automatically by Diesel CLI.

diesel::table! {
    categorias (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 30]
        nome -> Varchar,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    clientes (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 2]
        tipoPessoa -> Bpchar,
        #[max_length = 11]
        cpf -> Nullable<Varchar>,
        #[max_length = 14]
        cnpj -> Nullable<Varchar>,
        #[max_length = 60]
        nome -> Varchar,
        #[max_length = 14]
        ie -> Nullable<Varchar>,
        #[max_length = 60]
        razaoSocial -> Nullable<Varchar>,
        dataNascimento -> Date,
        #[max_length = 1]
        sexo -> Bpchar,
        email -> Text,
        #[max_length = 14]
        telefone -> Varchar,
        senha -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    enderecosEntrega (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        idPedido -> Varchar,
        #[max_length = 120]
        nomeRemetente -> Varchar,
        #[max_length = 8]
        cep -> Bpchar,
        #[max_length = 60]
        logradouro -> Varchar,
        #[max_length = 10]
        numero -> Varchar,
        #[max_length = 60]
        complemento -> Nullable<Varchar>,
        #[max_length = 60]
        bairro -> Varchar,
        #[max_length = 7]
        codigoIbgeCidade -> Bpchar,
        #[max_length = 2]
        codigoIbgeUF -> Bpchar,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    pagamentos (id) {
        id -> Text,
        #[max_length = 36]
        idPedido -> Varchar,
        #[max_length = 1]
        formaPagamento -> Varchar,
        numeroParcelas -> Int2,
        valorParcela -> Numeric,
        valorTotal -> Numeric,
        #[max_length = 48]
        boleto -> Nullable<Varchar>,
        pix -> Nullable<Text>,
        tid -> Nullable<Text>,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    pedidos (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        idCliente -> Varchar,
        valorBruto -> Numeric,
        valorFrete -> Numeric,
        valorDesconto -> Numeric,
        valorLiquido -> Numeric,
        #[max_length = 1]
        status -> Bpchar,
        dataEntrega -> Date,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    produtos (sku) {
        sku -> Text,
        codigo -> Int4,
        #[max_length = 36]
        idCategoria -> Varchar,
        #[max_length = 120]
        nome -> Varchar,
        descricao -> Nullable<Text>,
        foto -> Nullable<Text>,
        preco -> Numeric,
        estoque -> Numeric,
        pctoferta -> Numeric,
        qtdvendas -> Int4,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    produtosPedido (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        idPedido -> Varchar,
        skuProduto -> Text,
        quantidade -> Numeric,
        valorUnitario -> Numeric,
        valorBruto -> Numeric,
        valorFrete -> Numeric,
        valorDesconto -> Numeric,
        valorLiquido -> Numeric,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::joinable!(enderecosEntrega -> pedidos (idPedido));
diesel::joinable!(pagamentos -> pedidos (idPedido));
diesel::joinable!(pedidos -> clientes (idCliente));
diesel::joinable!(produtos -> categorias (idCategoria));
diesel::joinable!(produtosPedido -> pedidos (idPedido));
diesel::joinable!(produtosPedido -> produtos (skuProduto));

diesel::allow_tables_to_appear_in_same_query!(
    categorias,
    clientes,
    enderecosEntrega,
    pagamentos,
    pedidos,
    produtos,
    produtosPedido,
);
