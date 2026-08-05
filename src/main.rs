/*
*** Objectif
* Projet proposé par rust book à la base : rechercher une chaine de chars dans un fichier texte simple.

*** Apprentissage :
* La notion de cursor et comment l'employer.
* Organiser un projet

*** Au final :
* J'ai appris à utiliser Iter, un peu de LINQ. Ma première version était complexe mais m'a appris le trait iter.
* Celle corrigée avec l'ia a changé mon point de vue POO pour quelque chose de plus simple au final.
*/

use std::{env, fs, process::exit};

mod searcher;
use searcher::{search};

const ARGS_LENGTH : usize = 3;
const QUERY_ARG_INDEX : usize = 1;
const FILE_PATH_ARG_INDEX : usize = 2;

fn main() {
    let args : Vec<String> = env
        ::args()
        .collect();

    if args.len() != ARGS_LENGTH {
        eprintln!("Use : <run> <query> <file path>");
        exit(1);
    }

    let path = &args[FILE_PATH_ARG_INDEX];

    let file_content = fs
        ::read_to_string(path)
        .expect("Le chemin du fichier est incorrecte.");


    let query = &args[QUERY_ARG_INDEX];
    for line in search(query, &file_content) {
        println!("L.{} : {}", line.number, line.content);
    }
}
