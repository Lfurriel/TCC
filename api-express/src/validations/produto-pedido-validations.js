import { z } from "zod";

export const createProdutoPedidoSchema = z.object({
    skuProduto: z.string().trim().min(1),
    quantidade: z.number().gt(0).int(),
})