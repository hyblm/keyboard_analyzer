use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let layout = {
        let mut buf = String::new();
        let mut layout_file = File::open("layouts/dhorf").unwrap();
        layout_file.read_to_string(&mut buf).unwrap();
        buf
    };
    let character_filter: String = layout.chars().filter(|c| !c.is_whitespace()).collect();

    let corpus_string: String = {
        let mut buf = String::new();
        let mut corpus_file = File::open("corpora/english_10k.json").unwrap();
        corpus_file.read_to_string(&mut buf).unwrap();
        buf
    };

    let start = corpus_string.find('[').unwrap() + 7;
    let end = corpus_string.find(']').unwrap() - 4;
    let corpus: Vec<char> = corpus_string[start..end]
        .split('"')
        .flat_map(|s| s.trim().chars())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    // print!("{corpus:?}");
    let mut bigrams: HashMap<[char; 2], usize> = HashMap::new();
    for word in corpus.split(|&c| c == ',') {
        for (i, bigram) in word.windows(2).enumerate() {
            if i > 5 {
                break;
            }
            let entry = bigrams.entry(bigram.try_into().unwrap()).or_insert(0);
            *entry += 1;
            // println!("{bigram:?} = {entry:?}");
        }
    }

    let mut bigrams_vec: Vec<_> = bigrams.iter().collect();
    bigrams_vec.sort_unstable_by_key(|&(_, count)| usize::MAX.saturating_sub(*count));
    dbg!((26 * 26 * 2) - dbg!(bigrams_vec.len()));
    for (i, (bigram, count)) in bigrams_vec.iter().enumerate() {
        if i > 5 {
            break;
        }
        println!("{bigram:?} = {count}");
    }
    println!("done");
}
