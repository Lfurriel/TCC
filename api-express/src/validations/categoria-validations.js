import { z } from "zod";

export const createCategoriaSchema = z.object({
    nome: z.string().trim().min(1).max(30),
})

export const updateCategoriaSchema = createCategoriaSchema.partial();