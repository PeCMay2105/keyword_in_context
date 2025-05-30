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

                let keywords = find_keyWords(&palavras, &STOPWORDS);    
                for keyword in keywords {
                    all_keywords.insert(keyword);
                }
            }

            // Executa KWIC para cada keyword única
            // Ordena as keywords antes de iterar
            let mut sorted_keywords: Vec<String> = all_keywords.into_iter().collect();
            sorted_keywords.sort();

            for keyword in sorted_keywords {
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


pub static STOPWORDS: Lazy<HashSet<String>> = Lazy::new(|| {
    let content = fs::read_to_string("stopWords.txt").unwrap_or_else(|_| {
        eprintln!("Aviso: Não foi possível processar o arquivo stopWords.txt. Usando conjunto vazio de stopwords.");
        String::new()
    });
    content
        .split(|c: char| c == ',' || c == '\n' || c == '\r')
        .map(|word| word.trim().to_lowercase())
        .filter(|word| !word.is_empty())
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

// passando as stop_words como parâmetro para os testes da função
pub fn find_keyWords(linha: &[String], stopwords_set: &HashSet<String>) -> Vec<String> {
    let mut keyWords = Vec::<String>::new();
    for palavra in linha {
        
        if !stopwords_set.contains(palavra) {
            keyWords.push(palavra.clone());
        }
    }
    return keyWords;
}




pub struct KwicSystem{
    pub lines: Vec<String>,
    pub pos_index: HashMap<String,Vec<usize>>,
}

#[derive(Debug, PartialEq)] // para usar nos testes
pub struct KwicResult{
    pub n_line: usize,
    pub key_word: String,
    pub right_context: String,
    pub left_context: String,
    pub line: String,


}

impl fmt::Display for KwicResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} **{}** {}",
            self.key_word,
            self.key_word,
            self.right_context,
            self.left_context,
            )
    }

}

// impl KwicResult {
//     pub fn new() -> Self{
//         KwicResult { n_line: 0, left_context: (), key_word: (), right_context: (), line: () }
//     }
// }


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
                key_word: palavra.to_string(),
                right_context: String::from(right_context),
                left_context: String::from(left_context),  
                line: line.to_string()
            };
            resultado.push(individual_kwic);
        }
        }

        resultado
    }
}

#[cfg(test)]
mod tests {
    use std::string;
    use std::collections::{HashMap,HashSet};
    use super::*;

    #[test]
    fn normalize_test() {

        let kwic = KwicSystem::new();
        let string = "TeSTes #12312314";
        assert_eq!(kwic.normalize(string), "testes");
    }

    #[test]
    fn find_keywords_test() {

        let stopwords: HashSet<String> = HashSet::from(["de".to_string(), "o".to_string(), "a".to_string()]);        
        let linha: Vec<String> = vec!["testes".to_string(), "de".to_string(), "software".to_string()];

        assert_eq!(find_keyWords(&linha, &stopwords), ["testes", "software"]);
    }

    #[test]
    fn add_lines_test() {

        let mut kwic = KwicSystem::new();
        kwic.add_line("linha para teste".to_string());
        
        let linhas: Vec<String> = vec!["linha para teste".to_string()];
        
        assert_eq!(kwic.lines, linhas);
    }
    //==================================================
    // Testes para a função `normalize`
    //==================================================

    #[test]
    fn test_normalize_basic() {
        let kwic = KwicSystem::new();
        // Caso original: remove pontuação e números, converte para minúsculas
        assert_eq!(kwic.normalize("TeSTes #12312314"), "testes");
    }

    #[test]
    fn test_normalize_with_hyphen_and_special_chars() {
        let kwic = KwicSystem::new();
        // O comportamento atual junta palavras com hífen. É importante testar e estar ciente disso.
        assert_eq!(kwic.normalize("palavra-com-hífen"), "palavracomhífen");
        assert_eq!(kwic.normalize("don't"), "dont");
    }

    #[test]
    fn test_normalize_empty_and_no_alphabetic() {
        let kwic = KwicSystem::new();
        // Testando strings que resultam em uma string vazia
        assert_eq!(kwic.normalize(""), "");
        assert_eq!(kwic.normalize("123 !@#$ %^&*"), "");
    }
    
    #[test]
    fn test_normalize_unicode() {
        let kwic = KwicSystem::new();
        // Garante que caracteres acentuados (que são alfabéticos) são mantidos
        assert_eq!(kwic.normalize("Ação"), "ação");
    }


    //==================================================
    // Testes para a função `find_keyWords`
    //==================================================
    
    #[test]
    fn test_find_keywords_basic() {
        let stopwords: HashSet<String> = HashSet::from(["de".to_string(), "o".to_string(), "a".to_string()]);
        let linha: Vec<String> = vec!["testes".to_string(), "de".to_string(), "software".to_string()];
        // Caso original
        assert_eq!(find_keyWords(&linha, &stopwords), vec!["testes".to_string(), "software".to_string()]);
    }

    #[test]
    fn test_find_keywords_all_stopwords() {
        let stopwords: HashSet<String> = HashSet::from(["de".to_string(), "o".to_string(), "a".to_string()]);
        let linha: Vec<String> = vec!["o".to_string(), "a".to_string(), "de".to_string()];
        // Nenhum keyword deve ser retornado
        assert!(find_keyWords(&linha, &stopwords).is_empty());
    }

    #[test]
    fn test_find_keywords_no_stopwords() {
        let stopwords: HashSet<String> = HashSet::from(["de".to_string(), "o".to_string(), "a".to_string()]);
        let linha: Vec<String> = vec!["rust".to_string(), "programming".to_string(), "language".to_string()];
        // Todas as palavras devem ser retornadas
        assert_eq!(find_keyWords(&linha, &stopwords), vec!["rust", "programming", "language"]);
    }
    
    #[test]
    fn test_find_keywords_empty_input() {
        let stopwords: HashSet<String> = HashSet::from(["de".to_string()]);
        let linha: Vec<String> = vec![];
        // Linha vazia deve retornar um vetor vazio
        assert!(find_keyWords(&linha, &stopwords).is_empty());
    }


    //==================================================
    // Testes para o `KwicSystem`
    //==================================================
    
    #[test]
    fn test_add_line_updates_index() {
        let mut kwic = KwicSystem::new();
        kwic.add_line("Primeira linha de teste.".to_string());
        kwic.add_line("Segunda linha.".to_string());

        // Verifica se as linhas foram adicionadas
        assert_eq!(kwic.lines.len(), 2);
        
        // Verifica se o índice de posições (`pos_index`) foi populado corretamente
        assert_eq!(kwic.pos_index.get("linha"), Some(&vec![0, 1]));
        assert_eq!(kwic.pos_index.get("teste"), Some(&vec![0]));
        assert_eq!(kwic.pos_index.get("segunda"), Some(&vec![1]));
        // Palavra que não existe
        assert_eq!(kwic.pos_index.get("naoexiste"), None);
    }
    
    #[test]
    fn test_search_keyword_not_found() {
        let mut kwic = KwicSystem::new();
        kwic.add_line("Uma linha qualquer.".to_string());
        let results = kwic.search_keyword("inexistente");
        // A busca por uma palavra que não está no índice deve retornar um vetor vazio
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_keyword_at_start_of_line() {
        let mut kwic = KwicSystem::new();
        kwic.add_line("Linha para teste de software.".to_string());
        
        let expected = vec![
            KwicResult {
                n_line: 0,
                left_context: "".to_string(),
                key_word: "Linha".to_string(),
                // Note o espaço no início do contexto direito
                right_context: " para teste de software.".to_string(),
                line: "Linha para teste de software.".to_string(),
            }
        ];
        
        let results = kwic.search_keyword("Linha");
        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_keyword_in_middle_of_line() {
        let mut kwic = KwicSystem::new();
        kwic.add_line("Uma linha para teste.".to_string());
        
        let expected = vec![
            KwicResult {
                n_line: 0,
                left_context: "uma linha ".to_string(),
                key_word: "para".to_string(),
                right_context: " teste.".to_string(),
                line: "Uma linha para teste.".to_string(),
            }
        ];
        
        let results = kwic.search_keyword("para");
        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_keyword_at_end_of_line() {
        let mut kwic = KwicSystem::new();
        kwic.add_line("Uma linha para teste".to_string());
        
        let expected = vec![
            KwicResult {
                n_line: 0,
                left_context: "uma linha para ".to_string(),
                key_word: "teste".to_string(),
                right_context: "".to_string(),
                line: "Uma linha para teste".to_string(),
            }
        ];
        
        let results = kwic.search_keyword("teste");
        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_multiple_occurrences_in_different_lines() {
        let mut kwic = KwicSystem::new();
        kwic.add_line("O sistema é bom.".to_string()); // index 0
        kwic.add_line("Este é outro sistema.".to_string()); // index 1
        
        let expected = vec![
            KwicResult {
                n_line: 0,
                left_context: "o ".to_string(),
                key_word: "sistema".to_string(),
                right_context: " é bom.".to_string(),
                line: "O sistema é bom.".to_string(),
            },
            KwicResult {
                n_line: 1,
                left_context: "este é outro ".to_string(),
                key_word: "sistema".to_string(),
                right_context: ".".to_string(),
                line: "Este é outro sistema.".to_string(),
            },
        ];
        
        let results = kwic.search_keyword("sistema");
        assert_eq!(results.len(), 2);
        assert_eq!(results, expected);
    }

    #[test]
    fn test_search_exposes_limitation_of_single_find_per_line() {
        let mut kwic = KwicSystem::new();
        // Esta linha contém a palavra "teste" duas vezes.
        kwic.add_line("um teste para outro teste".to_string());
        
        let expected = vec![
            KwicResult {
                n_line: 0,
                left_context: "um ".to_string(),
                key_word: "teste".to_string(),
                right_context: " para outro teste".to_string(),
                line: "um teste para outro teste".to_string(),
            }
        ];

        let results = kwic.search_keyword("teste");
        
        // A implementação atual com `find()` só encontra a PRIMEIRA ocorrência na linha.
        // Um sistema KWIC completo deveria encontrar ambas.
        // Este teste confirma o comportamento atual e expõe essa limitação.
        assert_eq!(results.len(), 1, "Atenção: Apenas a primeira ocorrência da keyword na linha foi encontrada.");
        assert_eq!(results, expected);
    }
}

    // #[test]
    // fn search_keyword_test() {

    //     let mut kwic = KwicSystem::new();
    //     kwic.add_line("linha para teste".to_string());
        
    //     let linhas: Vec<String> = vec!["linha para teste".to_string()];

    //     let result = KwicResult {
    //         n_line: 0,
    //         left_context: "".to_string(),
    //         key_word: "linha".to_string(),
    //         right_context: "para teste".to_string(),
    //         line: "linha para teste".to_string(),
    //     };
        
    //     let result_vector: Vec<KwicResult> = vec![result]; 
    //     let kwic_vector = kwic.search_keyword("linha");
        
        
    //     assert_eq!(kwic_vector[0], result_vector[0]);        
    // }
