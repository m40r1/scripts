extern crate csv;
extern crate serde;
//this makes #[derive(Deserialize)] possible
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::fmt::Debug;
use std::ops::Not;
use std::usize;
use std::{env, ffi::OsString, process};

/// calls the formating fn
/// this is made according to the db data model
fn formatar_csv(csv: &mut Csv) {
    csv.escreve_null();
    //this puts an ( before the first record
    csv.cod_empresa.insert(0, '(');
    csv.convert_nome_perg_str();
    csv.convert_rg_perg_str();
    csv.limpa_cpf();
    csv.convert_nasc_perg_str();
    csv.convert_rua_perg_str();
    csv.convert_bairro_perg_str();
    csv.convert_cidade_perg_str();
    csv.convert_estado_perg_str();
    csv.convert_telefone_perg_str();
    csv.convert_celular_perg_str();
    csv.convert_email_perg_str();
    csv.convert_sexo_perg_str();
    csv.convert_recebe_email_perg_str();
    //this puts as ), after the record
    csv.convert_apt_perg_str();
}
/// read and write logic here
fn run() -> Result<(), Box<dyn Error>> {
    let file_path = primeiro_arg()?;
    let out_path = segundo_arg()?;

    //tweak the csv reader here
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .flexible(true)
        .from_path(file_path)?;
    // tweak the writer here
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Never)
        .has_headers(true)
        .flexible(true)
        .from_path(out_path)?;
    // loop the csv file
    // write according to the csv struct
    for result in rdr.deserialize() {
        let mut csv: Csv = result?;
        //formato o csv junto com a leitura
        formatar_csv(&mut csv);
        //escreve o csv formatado
        wtr.serialize(&csv)?;
    }
    // cleans the write
    wtr.flush()?;
    Ok(())
}
// pega o primeiro argumento
// pode ser um nome de arquivo
// ou o caminho completo
fn primeiro_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("passe apenas o camino do arquivo")),
        Some(file_path) => Ok(file_path),
    }
}
fn segundo_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(2) {
        None => Err(From::from("caminho do arquivo de output")),
        Some(file_path) => Ok(file_path),
    }
}
/// check errs in the run fn
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1)
    }
}
/// csv structure
/// use option if the record could be empty
#[derive(Debug, Deserialize, Serialize)]
struct Csv {
    cod_empresa: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    cod_pessoa: Option<u32>,
    nome: String,
    situacao: String,
    rg: Option<String>,
    cpf: Option<String>,
    nascimento: String,
    rua: String,
    bairro: String,
    cidade: String,
    estado: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    cep: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefone: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    celular: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    email: Option<String>,
    sexo: String,
    nacionalidade: u8,
    cod_categ_usuario: u32,
    recebe_email: String,
    cod_tipo_emprestimo: u32,
    #[serde(deserialize_with = "csv::invalid_option")]
    numero: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    apt: Option<String>,
}

/// formating to db needs
impl Csv {
    /// if the record is empty writes "NULL"
    fn escreve_null(&mut self) {
        if self.apt.is_none() {
            self.apt = Some(String::from("NULL"));
        }

        if self.numero.is_none() {
            self.numero = Some(0);
        }

        if self.email.is_none() {
            self.email = Some(String::from("NULL"));
        }

        if self.celular.is_none() {
            self.celular = Some(String::from("NULL"));
        }

        if self.telefone.is_none() {
            self.telefone = Some(String::from("NULL"));
        }

        if self.cep.is_none() {
            self.cep = Some(String::from("NULL"));
        }

        if self.rg.is_none() {
            self.rg = Some(String::from("NULL"));
        }

        if self.cpf.is_none() {
            self.cpf = Some(String::from("NULL"));
        }
    }
    /// string no sql server = ' str '
    /// you can just copy & paste this
    /// maybe later i make this a generic function
    /// but i dont know how yet
    fn convert_nome_perg_str(&mut self) {
        // some basic checks
        if self.nome.len() > 0 && self.nome != "NULL" {
            // se nao tiver com o padrao de str do pergamum
            // formata o nome
            if self.nome.starts_with('\'').not() {
                self.nome.insert_str(0, "\'");
                self.nome.push_str("\'");
            }
        }
    }

    fn convert_rg_perg_str(&mut self) {
        let mut test = String::from("none");
        let a = self.rg.as_mut().unwrap_or(&mut test);
        if a.len() > 0 && a != "NULL" {
            if a.starts_with('\'').not() {
                a.insert_str(0, "\'");
                a.push_str("\'");
            }
        }
    }
    /// cpf.len must be 11
    /// i receive the cpf with extra stuff
    fn limpa_cpf(&mut self) {
        let mut test = String::from("none");
        let a = self.cpf.as_mut().unwrap_or(&mut test);
        if a.len() > 0 && a != "NULL" {
            // if its not the right size
            // i already know where the separators will be
            // remove changes the index
            // so be smart
            if a.len() != 11 {
                a.remove(3);
                a.remove(6);
                a.remove(9);
            }
            if a.starts_with('\'').not() {
                a.insert_str(0, "\'");
                a.push_str("\'");
            }
        }
    }

    fn convert_nasc_perg_str(&mut self) {
        if self.nascimento.len() > 0 && self.nascimento != "NULL" {
            if self.nascimento.starts_with('\'').not() {
                self.nascimento.insert_str(0, "\'");
                self.nascimento.push_str("\'");
            }
        }
    }

    fn convert_rua_perg_str(&mut self) {
        if self.rua.len() > 0 && self.rua != "NULL" {
            if self.rua.starts_with('\'').not() {
                self.rua.insert_str(0, "\'");
                self.rua.push_str("\'");
            }
        }
    }

    fn convert_bairro_perg_str(&mut self) {
        if self.bairro.len() > 0 && self.bairro != "NULL" {
            if self.bairro.starts_with('\'').not() {
                self.bairro.insert_str(0, "\'");
                self.bairro.push_str("\'");
            }
        }
    }

    fn convert_cidade_perg_str(&mut self) {
        if self.cidade.len() > 0 && self.cidade != "NULL" {
            if self.cidade.starts_with('\'').not() {
                self.cidade.insert_str(0, "\'");
                self.cidade.push_str("\'");
            }
        }
    }

    fn convert_estado_perg_str(&mut self) {
        if self.estado.len() > 0 && self.estado != "NULL" {
            if self.estado.starts_with('\'').not() {
                self.estado.insert_str(0, "\'");
                self.estado.push_str("\'");
            }
        }
    }

    fn convert_celular_perg_str(&mut self) {
        let mut test = String::from("none");
        let a = self.celular.as_mut().unwrap_or(&mut test);
        if a.len() > 0 && a != "NULL" {
            if a.starts_with('\'').not() {
                a.insert_str(0, "\'");
                a.push_str("\'");
            }
        }
    }

    fn convert_telefone_perg_str(&mut self) {
        let mut test = String::from("none");
        let a = self.telefone.as_mut().unwrap_or(&mut test);
        if a.len() > 0 && a != "NULL" {
            if a.starts_with('\'').not() {
                a.insert_str(0, "\'");
                a.push_str("\'");
            }
        }
    }

    fn convert_email_perg_str(&mut self) {
        let mut test = String::from("none");
        let a = self.email.as_mut().unwrap_or(&mut test);
        if a.len() > 0 && a != "NULL" {
            if a.starts_with('\'').not() {
                a.insert_str(0, "\'");
                a.push_str("\'");
            }
        }
    }

    fn convert_sexo_perg_str(&mut self) {
        if self.sexo.len() > 0 && self.sexo != "NULL" {
            if self.sexo.starts_with('\'').not() {
                self.sexo.insert_str(0, "\'");
                self.sexo.push_str("\'");
            }
        }
    }

    fn convert_recebe_email_perg_str(&mut self) {
        if self.recebe_email.len() > 0 && self.recebe_email != "NULL" {
            if self.recebe_email.starts_with('\'').not() {
                self.recebe_email.insert_str(0, "\'");
                self.recebe_email.push_str("\'");
            }
        }
    }

    fn convert_apt_perg_str(&mut self) {
        let mut test = String::from("none");
        let a = self.apt.as_mut().unwrap_or(&mut test);
        if a.len() > 0 && a != "NULL)," {
            if a.starts_with('\'').not() {
                a.insert_str(0, "\'");
                a.push_str("\'");
                a.push_str("),");
            }
        }
    }
}
