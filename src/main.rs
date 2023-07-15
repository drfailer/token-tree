use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
enum Tokens {
    WHILE,
    FOR,
    FOREACH,
    IF,
    INT,
    VOID,
    FN,
}

type RefNode = Rc<RefCell<TokenizerNode>>;

#[derive(Debug, Clone)]
// note: il faut utiliser RefCell et Rc ici
struct TokenizerNode {
    childs: HashMap<char, RefNode>,
    token: Option<Tokens>,
}

// generate token tree like a prefix tree
fn generate_token_tree(tokens_ids: HashMap<String, Tokens>) -> RefNode {
    let root: RefNode = Rc::new(RefCell::new(TokenizerNode { childs: HashMap::new(), token: None }));

    for (token_id, token) in tokens_ids {
        let mut current_node: RefNode = Rc::clone(&root);

        for character in token_id.chars() {
            if !current_node.borrow().childs.contains_key(&character) {
                current_node.borrow_mut().childs.insert(
                    character,
                    Rc::new(RefCell::new(TokenizerNode {
                        childs: HashMap::new(),
                        token: None,
                    }))
                );
            }
            let tmp = Rc::clone(current_node.borrow().childs.get_key_value(&character).unwrap().1);
            current_node = Rc::clone(&tmp);
        }
        current_node.borrow_mut().token = Some(token);
    }
    root
}

fn print_indent(level: i32) {
    for _ in 0..level {
        print!("  ");
    }
}

fn pretty_print_tree(root: RefNode, level: i32) {
    for (character, node) in root.borrow().childs.iter() {
        print_indent(level);
        println!("{} -> {:?}: ", character, node.borrow().token);
        pretty_print_tree(Rc::clone(node), level + 1);
    }
}

fn main() {
    let mut tokens: HashMap<String, Tokens> = HashMap::new();
    tokens.insert(String::from("if"), Tokens::IF);
    tokens.insert(String::from("for"), Tokens::FOR);
    tokens.insert(String::from("foreach"), Tokens::FOREACH);
    tokens.insert(String::from("while"), Tokens::WHILE);
    tokens.insert(String::from("int"), Tokens::INT);
    tokens.insert(String::from("void"), Tokens::VOID);
    tokens.insert(String::from("fn"), Tokens::FN);

    let tree = generate_token_tree(tokens);
    // dbg!(tree);
    pretty_print_tree(tree, 0);
}
