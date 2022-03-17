use tinyjson::JsonValue;

#[derive(Debug, Clone)]
struct Tree {
    nodes: Vec<Node>,
    root: usize,
}

impl Tree {
    fn new() -> Self {
        Self {
            nodes: Default::default(),
            root: 0,
        }
    }

    fn insert(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn set_root(&mut self, new_root: usize) {
        self.root = new_root;
    }

    fn set_parent(&mut self, node_id: usize, parent: usize) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.parent = Some(parent);
        }
    }

    fn traverse_in_order(&self) -> impl Iterator<Item = (Node, usize, usize)> {
        self.in_order(None).into_iter()
    }

    fn in_order(&self, node_id: Option<usize>) -> Vec<(Node, usize, usize)> {
        let node_id = node_id.unwrap_or(self.root);
        let mut buffer = vec![];
        self.in_order_rec(node_id, 0, &mut buffer);
        buffer
    }

    fn in_order_rec(&self, node_id: usize, depth: usize, buffer: &mut Vec<(Node, usize, usize)>) {
        if let Some(node) = self.nodes.get(node_id) {
            match node.value {
                Value::Number(_) => buffer.push((node.clone(), depth, node_id)),
                Value::Branch(left, right) => {
                    self.in_order_rec(left, depth + 1, buffer);
                    buffer.push((node.clone(), depth, node_id));
                    self.in_order_rec(right, depth + 1, buffer);
                }
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    parent: Option<usize>,
    value: Value,
}

impl Node {
    fn new(value: Value, parent: Option<usize>) -> Self {
        Self { value, parent }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    Branch(usize, usize),
}

impl Value {
    fn number(&self) -> Option<f64> {
        match self {
            Value::Number(f) => Some(*f),
            _ => None,
        }
    }

    fn children(&self) -> Option<(usize, usize)> {
        match self {
            Value::Branch(left, right) => Some((*left, *right)),
            _ => None,
        }
    }

    fn set_number(&mut self, value: f64) {
        if let Value::Number(x) = self {
            *x = value;
        }
    }
}

fn main() {
    let trees: Vec<Tree> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<JsonValue>().unwrap())
        .map(|json| {
            let mut tree = Tree::new();
            json_to_tree(&mut tree, None, json);
            tree
        })
        .collect();

    let magnitude = part1(trees.clone());
    println!("Part 1: {}", magnitude);
    assert_eq!(magnitude, 2501);

    let max_magnitude = part2(trees);
    println!("Part 2: {}", max_magnitude);
    assert_eq!(max_magnitude, 4935);
}

fn part1(trees: Vec<Tree>) -> usize {
    let acc = {
        let mut acc = trees.first().cloned().unwrap();
        for tree in trees.into_iter().skip(1) {
            let new_tree = merge_trees(acc.clone(), tree);
            acc = reduce(new_tree);
        }
        acc
    };
    magnitude(&acc, None)
}

fn part2(trees: Vec<Tree>) -> usize {
    let mut max_magnitude = 0;
    for x in 0..trees.len() {
        for y in 0..trees.len() {
            if x != y {
                let t1 = trees.get(x).unwrap();
                let t2 = trees.get(y).unwrap();
                let merged = merge_trees(t1.clone(), t2.clone());
                let reduced = reduce(merged);
                let magnitude = magnitude(&reduced, None);
                max_magnitude = max_magnitude.max(magnitude);
            }
        }
    }
    max_magnitude
}

fn merge_trees(left: Tree, right: Tree) -> Tree {
    let json = format!(
        "[{},{}]",
        tree_to_string(&left, None),
        tree_to_string(&right, None)
    );
    let json = json.parse::<JsonValue>().unwrap();
    let mut tree = Tree::new();
    json_to_tree(&mut tree, None, json);

    tree
}

fn tree_to_string(tree: &Tree, node_id: Option<usize>) -> String {
    if let Some(node) = tree.nodes.get(node_id.unwrap_or(tree.root)) {
        match node.value {
            Value::Branch(left, right) => {
                let s1 = tree_to_string(tree, Some(left));
                let s2 = tree_to_string(tree, Some(right));
                format!("[{},{}]", s1, s2)
            }
            Value::Number(n) => n.to_string(),
        }
    } else {
        unreachable!()
    }
}

fn magnitude(acc: &Tree, node_id: Option<usize>) -> usize {
    let node_id = node_id.unwrap_or(acc.root);
    if let Some(node) = acc.nodes.get(node_id) {
        match node.value {
            Value::Number(n) => n as usize,
            Value::Branch(left, right) => {
                magnitude(acc, Some(left)) * 3 + magnitude(acc, Some(right)) * 2
            }
        }
    } else {
        unreachable!()
    }
}

fn json_to_tree(tree: &mut Tree, parent: Option<usize>, json: JsonValue) -> usize {
    match json {
        JsonValue::Array(array) => {
            let left = json_to_tree(tree, None, array.first().unwrap().to_owned());
            let right = json_to_tree(tree, None, array.last().unwrap().to_owned());
            let parent = tree.insert(Node::new(Value::Branch(left, right), parent));
            tree.set_parent(left, parent);
            tree.set_parent(right, parent);
            tree.set_root(parent);
            parent
        }
        JsonValue::Number(n) => {
            let node = Node::new(Value::Number(n), parent);
            tree.insert(node)
        }
        _ => unreachable!(),
    }
}

fn reduce(tree: Tree) -> Tree {
    let (exploded, data) = explode(tree);
    if exploded {
        return reduce(data);
    }

    let (split, data) = split(data);
    if split {
        return reduce(data);
    }

    data
}

fn explode(mut tree: Tree) -> (bool, Tree) {
    let explodable = find_first_nested_node(&tree);

    if let Some(node_idx) = explodable {
        let (left_idx, right_idx) = tree.nodes.get(node_idx).unwrap().value.children().unwrap();
        let left_value = tree.nodes.get(left_idx).unwrap().value.number().unwrap();
        let right_value = tree.nodes.get(right_idx).unwrap().value.number().unwrap();

        if let Some(left_neighbour) = find_neighbour_on_left_side(&tree, left_idx) {
            if let Some(left_node) = tree.nodes.get_mut(left_neighbour) {
                left_node
                    .value
                    .set_number(left_node.value.number().unwrap() + left_value);
            }
        }
        if let Some(right_neighbour) = find_neighbour_on_right_side(&tree, right_idx) {
            if let Some(right_node) = tree.nodes.get_mut(right_neighbour) {
                right_node
                    .value
                    .set_number(right_node.value.number().unwrap() + right_value);
            }
        }

        let exploded_node = tree.nodes.get_mut(node_idx).unwrap();
        exploded_node.value = Value::Number(0f64);

        return (true, tree);
    }

    (false, tree)
}

fn find_first_nested_node(tree: &Tree) -> Option<usize> {
    for (node, depth, node_id) in tree.traverse_in_order() {
        if let Value::Branch(l, r) = node.value {
            if depth >= 4 {
                let left = tree.nodes.get(l).unwrap();
                let right = tree.nodes.get(r).unwrap();
                if let (Value::Number(_), Value::Number(_)) = (&left.value, &right.value) {
                    return Some(node_id);
                }
            }
        }
    }
    None
}

fn find_neighbour_on_left_side(tree: &Tree, node_id: usize) -> Option<usize> {
    let mut last_node_idx = None;
    for (node, _, idx) in tree.traverse_in_order() {
        if idx == node_id {
            break;
        } else if let Value::Number(_) = node.value {
            last_node_idx = Some(idx)
        }
    }
    last_node_idx
}

fn find_neighbour_on_right_side(tree: &Tree, node_id: usize) -> Option<usize> {
    let mut passed = false;
    for (node, _, idx) in tree.traverse_in_order() {
        if passed {
            if let Value::Number(_) = node.value {
                return Some(idx);
            }
        }

        if idx == node_id {
            passed = true;
        }
    }
    None
}

fn split(mut tree: Tree) -> (bool, Tree) {
    for (node, _, node_id) in tree.traverse_in_order() {
        if let Value::Number(n) = node.value {
            if n >= 10f64 {
                let left = Node::new(Value::Number(f64::floor(n / 2f64)), Some(node_id));
                let right = Node::new(Value::Number(f64::ceil(n / 2f64)), Some(node_id));

                let left_id = tree.insert(left);
                let right_id = tree.insert(right);

                let self_node = tree.nodes.get_mut(node_id).unwrap();
                self_node.value = Value::Branch(left_id, right_id);

                return (true, tree);
            }
        }
    }

    (false, tree)
}

#[cfg(test)]
mod tests {
    use tinyjson::JsonValue;

    use crate::{explode, json_to_tree, split, tree_to_string, Tree};

    #[test]
    fn test_explode() {
        let tree = "[[[[[9,8],1],2],3],4]"
            .lines()
            .next()
            .map(|line| line.parse::<JsonValue>().unwrap())
            .map(|json| {
                let mut tree = Tree::new();
                json_to_tree(&mut tree, None, json);
                tree
            })
            .unwrap();
        let (exploded, new_tree) = explode(tree);
        assert!(exploded);
        assert_eq!(
            "[[[[0,9],2],3],4]".to_string(),
            tree_to_string(&new_tree, None)
        );
    }

    #[test]
    fn test_split() {
        let tree = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
            .lines()
            .next()
            .map(|line| line.parse::<JsonValue>().unwrap())
            .map(|json| {
                let mut tree = Tree::new();
                json_to_tree(&mut tree, None, json);
                tree
            })
            .unwrap();
        let (split, new_tree) = split(tree);
        assert!(split);
        assert_eq!(
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string(),
            tree_to_string(&new_tree, None)
        );
    }
}
