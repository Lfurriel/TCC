import { z } from "zod";

export const createEnderecoEntregaSchema = z.object({
    nomeRemetente: z.string().trim().min(1).max(120),
    cep: z.string().regex(/^\d{8}$/),
    logradouro: z.string().trim().min(1).max(60),
    numero: z.string().trim().min(1).max(60),
    complemento: z.string().trim().min(1).max(60).optional(),
    bairro: z.string().trim().min(1).max(60),
    codigoIbgeCidade: z.string().regex(/^\d{7}$/),
    codigoIbgeUF: z.string().regex(/^\d{2}$/),
})