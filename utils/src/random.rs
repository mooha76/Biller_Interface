use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_string(len:usize)->String{
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()

}

pub fn generate_random_string_with_prefix(prefix:&str , len:usize)->String{
    format!("{prefix}-{}",generate_random_string(len))
}


#[cfg(test)]
mod test{
    use fake::{Fake, Faker};

    use super::*;
    #[test]

    fn test_generate_random_string(){
        let len =5;
        let random =generate_random_string(len);
        println!("generated string= {}",random);
        assert_eq!(random.len() , len)

    }
    #[test]
    fn test_generate_random_string_with_prefix(){
        let prefix="WEB";
        let len=10;
        let generated_random=generate_random_string_with_prefix(prefix ,len);
        println!("Generated Random with prefix= {}", generated_random);
        assert!(generated_random.starts_with(&*prefix))
    }
}