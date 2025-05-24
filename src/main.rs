use contracts::*;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


pub struct KwicSystem{
    pub lines: Vec<String>,
    pub pos_index: HashMap<String,Vec<usize>>,
}

pub struct KwicResult{
    pub n_line: usize,
    pub left_context: String,
    pub key_Word: String,
    pub right_context: String,
    pub line: String,
}

impl KwicSystem{

    #[ensures((ret.lines.is_empty()))]
    #[ensures(ret.pos_index.is_empty())]
    pub fn new() -> Self{
        KwicSystem{
            lines: Vec::new(),
            pos_index: HashMap<String,Vec<usize>>::new()
        }
    }

    #[requires(linha.len() > 0)]
    #[ensures(self.lines.len() == old(self.lines.len())+1)]
    pub fn add_line(&mut self, linha: String ){
        let index = self.lines.len();
        self.lines.push(linha.clone());
        self.index_line_words(&linha,index);
    }

    fn index_line_words(linha: &str, indice:usize){
        let parsedLine:Vec<&str>= linha.split_whitespace();

        for palavra in parsedLine{
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

    fn normalize(palavra:&str)-> String{
        let normalized = palavra
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();
        normalized
    }
    #[requires(palavra.len() > 0 && palavra != "")]
    pub fn search_keyword(&mut self,palavra: &str) -> Vec<KwicResult>{
        let normalized = normalize(&palavra);
        let index_lines = match self.pos_index.get(&normalized){
            Some(indices) => indices,
            None => return Vec<>::new()
        };
        let mut resultado = Vec<KwicResult>::new();
        for index in index_lines{
            let line = &self.lines[*index];
            if let Some(pos) = line.find(&normalized){

            let left_context = &line[0..pos];
            let right_context = &line[pos+normalized.len()..];

            let individual_kwic = KwicResult{
                n_line: *index as u32,
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