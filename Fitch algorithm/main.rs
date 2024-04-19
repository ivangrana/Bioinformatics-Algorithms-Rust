use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

// Estrutura que representa um nó na árvore filogenética
#[derive(Debug)]
struct TreeNode {
    sequence: HashSet<char>, // Sequência associada ao nó (DNA, RNA ou aminoácidos)
    left: Option<Rc<RefCell<TreeNode>>>, // Filho esquerdo (opcional)
    right: Option<Rc<RefCell<TreeNode>>>, // Filho direito (opcional)
}

// Construtor para criar um novo nó
impl TreeNode {
    fn new(sequence: HashSet<char>) -> Self {
        TreeNode {
            sequence,
            left: None,
            right: None,
        }
    }
}

// Função que executa o algoritmo de Fitch
fn fitch_algorithm(node: Rc<RefCell<TreeNode>>) -> HashSet<char> {
    let mut node_ref = node.borrow_mut();
    // Se for uma folha (sem filhos), retorna a sequência associada ao nó
    if node_ref.left.is_none() && node_ref.right.is_none() {
        return node_ref.sequence.clone();
    }

    // Propaga as sequências dos filhos para o nó atual
    let left_sequence = fitch_algorithm(node_ref.left.as_ref().unwrap().clone());
    let right_sequence = fitch_algorithm(node_ref.right.as_ref().unwrap().clone());

    // Calcular a interseção das sequências
    let intersection: HashSet<char> = left_sequence
        .intersection(&right_sequence)
        .cloned()
        .collect();

    // Se a interseção não for vazia, usar como sequência ancestral
    if !intersection.is_empty() {
        node_ref.sequence = intersection;
    } else {
        // Se a interseção for vazia, use a união das sequências dos filhos
        node_ref.sequence = left_sequence;
        node_ref.sequence.extend(right_sequence);
    }

    // Retorna a sequência estimada do nó
    node_ref.sequence.clone()
}

// Exemplo de uso
fn main() {
    // Cria uma árvore filogenética com raiz e filhos
    let root = Rc::new(RefCell::new(TreeNode::new(HashSet::new())));
    let left_child = Rc::new(RefCell::new(TreeNode::new(HashSet::from(['A', 'C', 'G','T']))));
    let right_child = Rc::new(RefCell::new(TreeNode::new(HashSet::from(['C','C', 'G', 'T']))));

    // Configura a árvore: root -> left_child, right_child
    root.borrow_mut().left = Some(left_child);
    root.borrow_mut().right = Some(right_child);

    // Executa o algoritmo de Fitch a partir da raiz
    let estimated_sequence = fitch_algorithm(root);

    // Imprime a sequência ancestral estimada na raiz
    println!("Sequência ancestral estimada na raiz: {:?}", estimated_sequence);
}
