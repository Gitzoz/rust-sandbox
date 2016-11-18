extern crate rand;
extern crate euclid;

use rand::Rng;

fn random(from: i32, to: i32) -> i32 { 
    rand::thread_rng().gen_range(from, to)
}

fn make_positiv_random_vec() -> Vec<i32> {    
    vec![random(1, 10), random(1, 10)]
}

fn make_negativ_random_vec() -> Vec<i32> {    
    vec![random(-10, -1), random(-10, -1)]
}

fn euclid(vec1: &Vec<i32>, vec2: &Vec<i32>) -> f32 {
    if vec1.len() != vec2.len() {
        panic!("For euclid both vectors must have the same size");
    }        

    let mut before_square_root: f32 = 0.0;
    for idx in 0..vec1.len() {
        before_square_root += (vec1[idx]-vec2[idx]).pow(2) as f32;
    }
    before_square_root.sqrt()
}

#[derive(Debug)]
struct NnClassifier {
    trained: Vec<(i32, Vec<i32>)>,
}

impl NnClassifier{
    fn fit(training_data: &Vec<(i32, Vec<i32>)>) -> NnClassifier {
        NnClassifier {
            trained: training_data.clone(),
        }
    }

    fn predict(&self, data: &Vec<i32>) -> i32 {
        println!("Will predict a label for data: {:?}", data);
        self.closest(data)
    }

    fn closest(&self, row: &Vec<i32>) -> i32 {
        let mut best_distance = (self.trained[0].0, euclid(row, &self.trained[0].1));
        for idx in 1..self.trained.len() {
            let current_dist = (self.trained[idx].0, euclid(row, &self.trained[idx].1));
            if current_dist.1 < best_distance.1{
                best_distance = current_dist
            }
        }
        best_distance.0
    }
}

fn main() {
    let training_set = vec![
        (0, make_positiv_random_vec()),
        (0, make_positiv_random_vec()),
        (0, make_positiv_random_vec()),
        (0, make_positiv_random_vec()),
        (0, make_positiv_random_vec()),
        (1, make_negativ_random_vec()),
        (1, make_negativ_random_vec()),
        (1, make_negativ_random_vec()),
        (1, make_negativ_random_vec()),
        (1, make_negativ_random_vec()),
    ];

    let nn_classifier = NnClassifier::fit(&training_set);
    
    println!("{:?}", nn_classifier);
    for x in 0..100 {
    let unpredicted = make_positiv_random_vec();
    let predicted = nn_classifier.predict(&unpredicted);
    println!("{:?}", predicted);
    }
    
    
}