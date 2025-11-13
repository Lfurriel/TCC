import { z } from "zod";
import { createProdutoPedidoSchema } from "./produto-pedido-validations.js";
import { createEnderecoEntregaSchema } from "./endereco-entrega-validations.js";
import { createPagamentoSchema } from "./pagamento-validations.js";

export const createPedidoSchema = z.object({
    produtos: z.array(createProdutoPedidoSchema),
    enderecoEntrega: createEnderecoEntregaSchema,
    pagamento: createPagamentoSchema,
});