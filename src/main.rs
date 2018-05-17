use std::collections::HashMap;
struct BinaryAdd {
    state: char,
    transition : HashMap<char, HashMap<(char, char), (char,char)>>
}

trait Transition {
    fn new() -> Self;
    fn add(&mut self, a: &str, b:&str) -> String; 
}

impl Transition for BinaryAdd {

    fn new() -> Self {
        let mut transition = HashMap::<char, HashMap<(char,char), (char,char)>>::new();
        let mut s0 = HashMap::<(char,char), (char,char)>::new();
        s0.insert(('0','0'), ('0','0')); // a, b, z, next state
        s0.insert(('0','1'), ('1','0'));
        s0.insert(('1','0'), ('1','0'));
        s0.insert(('1','1'), ('0','1'));
        let mut s1 = HashMap::<(char,char), (char,char)>::new();
        s1.insert(('0','0'), ('1','0')); // a, b, z, next state
        s1.insert(('0','1'), ('0','1'));
        s1.insert(('1','0'), ('0','1'));
        s1.insert(('1','1'), ('1','1'));
        transition.insert('0', s0);
        transition.insert('1', s1);
        BinaryAdd {
            state: '0',
            transition: transition,
        }
    }
    fn add(&mut self, a:&str, b:&str) -> String{ 
        let len_diff = a.chars().count() as i32 - b.chars().count() as i32;
        let mut a = a.chars().rev().collect::<String>();
        let mut b = b.chars().rev().collect::<String>();
        let mut result = String::from("");
        if len_diff > 0{
            for _ in 0..len_diff{
                b.push('0');
            }
        }
        if len_diff < 0{
            for _ in 0..len_diff.abs(){
                a.push('0');
            }
        }
        for (x, y) in a.chars().zip(b.chars()){
            let (value, transaction) = self.transition.get(&self.state).unwrap().get(&(x,y)).unwrap();
            result.push(*value);
            self.state = *transaction;
        };
        let (value, _) = self.transition.get(&self.state).unwrap().get(&('0','0')).unwrap();
        result.push(*value);
        result.chars().rev().collect::<String>()
    }
}
fn main() {
    let mut binary_add = BinaryAdd::new();
    let result = binary_add.add("1100100100100", "100100011000");
    println!("{}", result);
}
