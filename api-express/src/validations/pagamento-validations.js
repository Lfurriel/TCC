import { z } from "zod";

export const createPagamentoSchema = z.object({
    formaPagamento: z.enum(['B', 'P', 'D', 'C']),
    numeroParcelas: z.number().int().gte(1).lte(12),
    porcentagemDesconto: z
        .number()
        .gte(0)
        .lte(1)
        .default(0)
        .refine(value => Number(value.toFixed(2)) === value, {
            message: "O desconto deve ter no máximo duas casas decimais",
        }),
});
