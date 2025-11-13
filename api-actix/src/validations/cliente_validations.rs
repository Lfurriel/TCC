use validator::ValidationError;
use crate::models::cliente::CreateClientePayload;

pub fn validate_cpf(cpf: &str) -> Result<(), ValidationError> {
    let cpf_clean = cpf.replace(|c: char| !c.is_ascii_digit(), "");

    if cpf_clean.len() != 11 {
        return Err(ValidationError::new("CPF deve ter 11 dígitos"));
    }

    if cpf_clean.chars().all(|c| c == cpf_clean.chars().next().unwrap()) {
        return Err(ValidationError::new("CPF inválido"));
    }

    let mut sum = 0;
    for (i, c) in cpf_clean.chars().take(9).enumerate() {
        sum += c.to_digit(10).unwrap() as i32 * (10 - i as i32);
    }
    let mut remainder = (sum * 10) % 11;
    if remainder == 10 || remainder == 11 {
        remainder = 0;
    }
    if remainder != cpf_clean.chars().nth(9).unwrap().to_digit(10).unwrap() as i32 {
        return Err(ValidationError::new("CPF inválido"));
    }

    sum = 0;
    for (i, c) in cpf_clean.chars().take(10).enumerate() {
        sum += c.to_digit(10).unwrap() as i32 * (11 - i as i32);
    }
    remainder = (sum * 10) % 11;
    if remainder == 10 || remainder == 11 {
        remainder = 0;
    }
    if remainder != cpf_clean.chars().nth(10).unwrap().to_digit(10).unwrap() as i32 {
        return Err(ValidationError::new("CPF inválido"));
    }

    Ok(())
}

pub fn validate_cnpj(cnpj: &str) -> Result<(), ValidationError> {
    let cnpj_clean = cnpj.replace(|c: char| !c.is_ascii_digit(), "");

    if cnpj_clean.len() != 14 {
        return Err(ValidationError::new("CNPJ deve ter 14 dígitos"));
    }

    if cnpj_clean.chars().all(|c| c == cnpj_clean.chars().next().unwrap()) {
        return Err(ValidationError::new("CNPJ inválido"));
    }

    let calc = |cnpj: &str, size: usize| -> i32 {
        let mut sum = 0;
        let weights = if size == 12 {
            vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]
        } else {
            vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]
        };

        for i in 0..size {
            sum += cnpj.chars().nth(i).unwrap().to_digit(10).unwrap() as i32 * weights[i];
        }
        let remainder = sum % 11;
        if remainder < 2 { 0 } else { 11 - remainder }
    };

    let digit1 = calc(&cnpj_clean, 12);
    let digit2 = calc(&cnpj_clean, 13);

    if digit1 != cnpj_clean.chars().nth(12).unwrap().to_digit(10).unwrap() as i32
        || digit2 != cnpj_clean.chars().nth(13).unwrap().to_digit(10).unwrap() as i32 {
        return Err(ValidationError::new("CNPJ inválido"));
    }

    Ok(())
}

pub fn validate_senha_forte(senha: &str) -> Result<(), ValidationError> {
    if senha.len() < 8 {
        return Err(ValidationError::new("A senha deve ter pelo menos 8 caracteres"));
    }

    if !senha.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("A senha deve conter pelo menos uma letra maiúscula"));
    }

    if !senha.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("A senha deve conter pelo menos uma letra minúscula"));
    }

    if !senha.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("A senha deve conter pelo menos um número"));
    }

    if !senha.chars().any(|c| !c.is_alphanumeric()) {
        return Err(ValidationError::new("A senha deve conter pelo menos um caractere especial"));
    }

    Ok(())
}

pub fn validate_telefone(telefone: &str) -> Result<(), ValidationError> {
    let digits_only: String = telefone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits_only.len() < 10 || digits_only.len() > 14 {
        return Err(ValidationError::new("Telefone deve ter entre 10 e 14 dígitos"));
    }
    Ok(())
}

pub fn validate_data_nascimento(data: &str) -> Result<(), ValidationError> {
    match chrono::NaiveDate::parse_from_str(data, "%Y-%m-%d") {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Data de nascimento inválida (formato: YYYY-MM-DD)"))
    }
}

pub fn validate_ie(ie: &str) -> Result<(), ValidationError> {
    if !ie.is_empty() && (ie.len() < 7 || ie.len() > 14) {
        return Err(ValidationError::new("IE deve ter entre 7 e 14 caracteres"));
    }
    Ok(())
}

pub fn validate_tipo_pessoa_consistency(payload: &CreateClientePayload) -> Result<(), ValidationError> {
    match payload.tipo_pessoa.as_str() {
        "PF" => {
            if payload.cpf.is_none() || payload.cpf.as_ref().unwrap().is_empty() {
                return Err(ValidationError::new("CPF é obrigatório para pessoa física"));
            }
            if payload.cnpj.is_some() && !payload.cnpj.as_ref().unwrap().is_empty() {
                return Err(ValidationError::new("CNPJ não deve ser informado para pessoa física"));
            }
        }
        "PJ" => {
            if payload.cnpj.is_none() || payload.cnpj.as_ref().unwrap().is_empty() {
                return Err(ValidationError::new("CNPJ é obrigatório para pessoa jurídica"));
            }
            if payload.cpf.is_some() && !payload.cpf.as_ref().unwrap().is_empty() {
                return Err(ValidationError::new("CPF não deve ser informado para pessoa jurídica"));
            }
            if payload.razao_social.is_none() || payload.razao_social.as_ref().unwrap().is_empty() {
                return Err(ValidationError::new("Razão social é obrigatória para pessoa jurídica"));
            }
        }
        _ => return Err(ValidationError::new("Tipo de pessoa deve ser PF ou PJ"))
    }
    Ok(())
}

pub fn validate_tipo_pessoa(tipo_pessoa: &str) -> Result<(), ValidationError> {
    if !["PF", "PJ"].contains(&tipo_pessoa) {
        return Err(ValidationError::new("Tipo de pessoa deve ser PF ou PJ"));
    }
    Ok(())
}

pub fn validate_sexo(sexo: &str) -> Result<(), ValidationError> {
    if !["M", "F", "N"].contains(&sexo) {
        return Err(ValidationError::new("Sexo deve ser M, F ou N"));
    }
    Ok(())
}