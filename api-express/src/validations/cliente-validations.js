import { z } from "zod";

// Função para validar CPF
const isValidCPF = (cpf) => {
  cpf = cpf.replace(/\D/g, ""); // Remove caracteres não numéricos
  if (cpf.length !== 11 || /^(\d)\1{10}$/.test(cpf)) return false; // Verifica formato e sequências repetidas

  let sum = 0;
  for (let i = 0; i < 9; i++) sum += parseInt(cpf.charAt(i)) * (10 - i);
  let remainder = (sum * 10) % 11;
  if (remainder === 10 || remainder === 11) remainder = 0;
  if (remainder !== parseInt(cpf.charAt(9))) return false;

  sum = 0;
  for (let i = 0; i < 10; i++) sum += parseInt(cpf.charAt(i)) * (11 - i);
  remainder = (sum * 10) % 11;
  if (remainder === 10 || remainder === 11) remainder = 0;
  return remainder === parseInt(cpf.charAt(10));
};

// Função para validar CNPJ
const isValidCNPJ = (cnpj) => {
  cnpj = cnpj.replace(/\D/g, ""); // Remove caracteres não numéricos
  if (cnpj.length !== 14 || /^(\d)\1{13}$/.test(cnpj)) return false; // Verifica formato e sequências repetidas

  const calc = (cnpj, size) => {
    let sum = 0;
    const weights = size === 12 ? [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2] : [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    for (let i = 0; i < size; i++) sum += parseInt(cnpj.charAt(i)) * weights[i];
    let remainder = sum % 11;
    return remainder < 2 ? 0 : 11 - remainder;
  };

  return calc(cnpj, 12) === parseInt(cnpj.charAt(12)) && calc(cnpj, 13) === parseInt(cnpj.charAt(13));
};

export const createClienteSchema = z
  .object({
    tipoPessoa: z.enum(["PF", "PJ"]),
    cpf: z
      .string()
      .trim()
      .optional()
      .refine((cpf) => !cpf || isValidCPF(cpf), { message: "CPF inválido" }),
    cnpj: z
      .string()
      .trim()
      .optional()
      .refine((cnpj) => !cnpj || isValidCNPJ(cnpj), { message: "CNPJ inválido" }),
    razaoSocial: z.string().trim().max(60).optional(),
    nome: z.string().trim().min(1).max(60),
    dataNascimento: z
      .string()
      .transform((val) => new Date(val))
      .refine((date) => !isNaN(date.getTime()), { message: "Invalid Date" }),
    ie: z.string().optional(),
    sexo: z.enum(["M", "F", "N"]),
    email: z.string().email(),
    telefone: z.string().trim().min(1).max(14),
    senha: z
      .string()
      .min(8, "A senha deve ter pelo menos 8 caracteres")
      .regex(/[A-Z]/, "A senha deve conter pelo menos uma letra maiúscula")
      .regex(/[a-z]/, "A senha deve conter pelo menos uma letra minúscula")
      .regex(/\d/, "A senha deve conter pelo menos um número")
      .regex(/[^A-Za-z0-9]/, "A senha deve conter pelo menos um caractere especial"),
    confirmarSenha: z.string(),
  })
  .refine(
    (data) => {
      if (data.tipoPessoa === "PF") {
        return data.cpf != undefined && !data.cnpj;
      }
      if (data.tipoPessoa === "PJ") {
        return data.cnpj != undefined && !data.cpf && !!data.razaoSocial;
      }
      return true;
    },
    {
      message: "Dados inconsistentes para o tipo de pessoa",
      path: ["tipoPessoa"],
    }
  )
  .refine((data) => data.senha === data.confirmarSenha, {
    message: "As senhas devem ser iguais",
    path: ["confirmarSenha"],
  })
  .transform((data) => ({
    ...data,
    razaoSocial: data.tipoPessoa === "PF" ? data.nome : data.razaoSocial,
  }));

export const loginSchema = z.object({
  email: z.string().email(),
  senha: z.string(),
})