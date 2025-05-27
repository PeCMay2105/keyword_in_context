use contracts::*;
use once_cell::sync::Lazy;
use std::collections::{HashMap,HashSet};
use std::io::{self, Write};
use std::fs;
use std::fmt;


fn main() {
    let mut kwic = KwicSystem::new();
    let mut all_keywords = HashSet::<String>::new();

    match std::fs::read_to_string("text.txt") {
        Ok(texto) => {
            // Adiciona linhas e coleta keywords únicas
            for linha in texto.lines() {
                kwic.add_line(linha.to_string());

                let palavras: Vec<String> = linha
                    .to_lowercase()
                    .split_whitespace()
                    .map(|word| word.to_string())
                    .collect();

                let keywords = find_keyWords(&palavras);
                for keyword in keywords {
                    all_keywords.insert(keyword);
                }
            }

            // Executa KWIC para cada keyword única
            for keyword in all_keywords {
                println!("=== Resultados para '{}' ===", keyword);
                let results = kwic.search_keyword(&keyword);

                for result in results {
                    println!("{}", result);
                }
                println!();
            }
        }
        Err(erro) => println!("Erro: {}", erro),
    }
}


pub static STOPWORDS: Lazy<HashSet<String>> = Lazy::new(||{
    let content = fs::read_to_string("stopWords.txt")
        .expect("Não foi possível processar o arquivo");
    content
        .lines()
        .map(|line| line.trim().to_lowercase())
        .collect()
});

//ainda é necessário adicionar essa função à main()
fn parse_files(path:&str) -> Vec<String>{
    let arquivo = std::fs::read_to_string(path);
    let mut lines = Vec::<String>::new();
    match arquivo{
        Ok(texto) =>{
            for linha in texto.lines(){
                lines.push(linha.to_string());
            }
        }
        Err(erro)=>{
            println!("Erro ao ler o arquivo: {}",erro);
        }
    }
    lines
}


pub fn find_keyWords(linha:&[String])-> Vec<String>{
    let mut keyWords = Vec::<String>::new();
    for palavra in linha{
        if !STOPWORDS.contains(palavra){
            keyWords.push(palavra.clone());
        }
    }
    return keyWords;
}



pub struct KwicSystem{
    pub lines: Vec<String>,
    pub pos_index: HashMap<String,Vec<usize>>,
}

pub struct KwicResult{
    pub n_line: usize,
    pub left_context: String,
    pub key_word: String,
    pub right_context: String,
    pub line: String,


}

impl fmt::Display for KwicResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}**{}**{}",
            self.key_word,
            self.right_context,
            self.left_context)
    }


}

impl KwicSystem{

    #[ensures((ret.lines.is_empty()))]
    #[ensures(ret.pos_index.is_empty())]
    pub fn new() -> Self{
        KwicSystem{
            lines: Vec::new(),
            pos_index: HashMap::new(),
        }
    }

    #[requires(linha.len() > 0)]
    #[ensures(self.lines.len() == old(self.lines.len())+1)]
    pub fn add_line(&mut self, linha: String ){
        let index = self.lines.len();
        self.lines.push(linha.clone());
        self.index_line_words(&linha,index);
    }

    fn index_line_words(&mut self, linha: &str, indice:usize){
        let parsed_line: Vec<&str> = linha.split_whitespace().collect();

        for palavra in parsed_line{
            let lower = palavra.to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

            self.pos_index
                .entry(lower)
                .or_insert_with(Vec::new)
                .push(indice);
        }
    }

    fn normalize(&self, palavra: &str) -> String{
        palavra
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
    }

    #[requires(palavra.len() > 0 && palavra != "")]
    pub fn search_keyword(&mut self, palavra: &str) -> Vec<KwicResult>{
        let normalized = self.normalize(palavra);
        let index_lines = match self.pos_index.get(&normalized){
            Some(indices) => indices,
            None => return Vec::new()
        };
        let mut resultado = Vec::<KwicResult>::new();
        for index in index_lines{
            let line = &self.lines[*index];
            let line_lower = line.to_lowercase();
            if let Some(pos) = line_lower.find(&normalized){

            let left_context = &line_lower[0..pos];
            let right_context = &line_lower[pos+normalized.len()..];

            let individual_kwic = KwicResult{
                n_line: *index,
                left_context: String::from(left_context),
                key_word: palavra.to_string(),
                right_context: String::from(right_context),
                line: line.to_string()
            };
            resultado.push(individual_kwic);
        }
        }

        resultado
    }
}