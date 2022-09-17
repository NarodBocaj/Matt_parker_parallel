use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,//all three aboved used to read in txt file
    collections::HashMap,
    sync::{Arc, Mutex},//used for syncing return vec of the looper to stop issues with multiple threads
    time::Instant,//used to get run time
};
use rayon::prelude::*;

fn main() {
    let now = Instant::now();
    let filename = "/Users/Jacob/Documents/Rusty things/Matt_Parker_Wordle/Matt_Parker_wordle_thing/video_words.txt";
    let mut num_word_map: HashMap<u32, String> = HashMap::new();
    let mut word_vec = lines_from_file(filename);
    word_vec.sort();

    let num_vec = encode(word_vec, &mut num_word_map);
    let ans_nums = looper(&num_vec, &mut num_word_map);
    
    let elapsed = now.elapsed();
    println!("DONE!");
    println!("Elapsed: {:.2?}", elapsed);
    //can probably get O(n^4) if we look at the xor of 26 1's and our solution up until the last one, then take 5 steps removing each one and check for them in a hashmap
    //link for stream
    //https://blog.yoshuawuyts.com/parallel-stream/
}

//reads in the text file
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

//converts the words to a number representation where in binary there is a one for any letter in the spot, z is in spot 25 a in spot 0. Removes words with duplicate letters and anagrams of words already in HashMap.
fn encode(word_vec: Vec<String>, num_word_map: &mut HashMap<u32, String>) -> Vec<u32>{
    let mut words_as_int = Vec::new();
    for word in word_vec{
        let mut temp = 0;

        for c in word.chars(){
            let mut n: u32 = c.into();
            n -= 97;
            let power = u32::pow(2, n);
            temp ^= power;
        }
        if temp.count_ones() == 5 && !num_word_map.contains_key(&temp){
            words_as_int.push(temp);
            num_word_map.insert(temp,word);
        }
    }
    return words_as_int
}

fn looper(num_vec: &[u32], num_word_map: & HashMap<u32, String>) -> Arc<Mutex<Vec<Vec<String>>>>{
    let n = num_vec.len();
    println!("Length of array is: {}", n);
    let mut ans_vec = Arc::new(Mutex::new(Vec::new()));
    (0..n-4).into_par_iter().for_each(|i|{
        //println!("{}",i);
        let a = num_vec[i];
        for j in i+1..n-3{
            let b = num_vec[j];
            if (a & b) != 0 {
                continue;
            }
            let ab = a | b;
            for k in j+1..n-2{
                let c = num_vec[k];
                if (ab & c) != 0{
                    continue;
                }
                let abc = ab | c;
                for l in k+1..n-1{
                    let d = num_vec[l];
                    if (abc & d) != 0{
                        continue;
                    }
                    let abcd = abc | d;
                    for m in l+1..n{
                        let e = num_vec[m];
                        if (abcd & e) != 0{
                            continue;
                        }
                        println!("adding an answer");
                        let temp_ans = decode(a,b,c,d,e, num_word_map);
                        for ans in &temp_ans{
                            println!("{}",ans);
                        }
                        ans_vec.lock().unwrap().push(temp_ans);
                    }
                }
            }
        }
    });
    return ans_vec;
}

fn decode(a:u32, b:u32, c:u32, d:u32, e:u32, num_word_map: & HashMap<u32, String>) -> Vec<String>{
    let mut ans = Vec::new();
    let ar: String = num_word_map.get(&a).expect("REASON").to_string();
    let br: String = num_word_map.get(&b).expect("REASON").to_string();
    let cr: String = num_word_map.get(&c).expect("REASON").to_string();
    let dr: String = num_word_map.get(&d).expect("REASON").to_string();
    let er: String = num_word_map.get(&e).expect("REASON").to_string();
    
    ans.push(ar);
    ans.push(br);
    ans.push(cr);
    ans.push(dr);
    ans.push(er);

    return ans;
}