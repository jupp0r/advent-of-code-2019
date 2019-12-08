use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{stdin, Error as IoError, Read};
use std::iter::Iterator;
use std::rc::Rc;

pub fn run() {
    let mut s = Vec::new();
    stdin()
        .read_to_end(&mut s)
        .expect("did not enter a correct string");
    let input_string = std::str::from_utf8(&s).expect("invalid utf8");

    let edge_strings = input_string.lines().collect::<Vec<&str>>();

    let roots = Tree::from_edge_strings(&edge_strings).unwrap();

    let num_parents: usize = roots
        .iter()
        .map(|tree| scan_bfs(tree.clone(), |state, info, _| state + info.depth, 0))
        .sum();
    println!("number of orbits: {}", num_parents);

    let root = roots.get(0).unwrap();

    println!(
        "roots: {:?}",
        roots
            .iter()
            .map(|n| n.borrow().name.clone())
            .collect::<Vec<String>>()
    );

    let (santa_node, _) = scan_bfs(
        root.clone(),
        |state, info, node| {
            if state.is_some() {
                return state;
            }

            if node.borrow().name == "SAN" {
                Some((node.clone(), info.depth))
            } else {
                None
            }
        },
        None,
    )
    .unwrap();

    let (you_node, _) = scan_bfs(
        root.clone(),
        |state, info, node| {
            if state.is_some() {
                return state;
            }

            if node.borrow().name == "YOU" {
                Some((node.clone(), info.depth))
            } else {
                None
            }
        },
        None,
    )
    .unwrap();

    let santa_parents = parents(santa_node);
    let you_parents = parents(you_node);

    let common_parents: Vec<Rc<RefCell<Tree>>> = santa_parents
        .iter()
        .filter(|n| {
            you_parents
                .iter()
                .find(|other| n.borrow().name == other.borrow().name)
                .is_some()
        })
        .cloned()
        .collect();

    let closest_ancestor_distance_from_santa = santa_parents
        .iter()
        .enumerate()
        .find(|(_, node)| {
            common_parents
                .iter()
                .find(|n| n.borrow().name == node.borrow().name)
                .is_some()
        })
        .map(|(i, _)| i + 1)
        .unwrap();
    let closest_ancestor_distance_from_you = you_parents
        .iter()
        .enumerate()
        .find(|(_, node)| {
            common_parents
                .iter()
                .find(|n| n.borrow().name == node.borrow().name)
                .is_some()
        })
        .map(|(i, _)| i + 1)
        .unwrap();

    println!(
        "{}",
        closest_ancestor_distance_from_santa + closest_ancestor_distance_from_you - 2,
    );
}

fn scan_bfs<F, State>(root: Rc<RefCell<Tree>>, mut visit: F, init: State) -> State
where
    F: FnMut(State, VisitationInfo, &Rc<RefCell<Tree>>) -> State,
{
    let mut unvisited: VecDeque<(Rc<RefCell<Tree>>, VisitationInfo)> = VecDeque::new();
    unvisited.push_back((root, VisitationInfo { depth: 0 }));

    let mut state = init;

    loop {
        if let Some((node, info)) = unvisited.pop_front() {
            state = visit(state, info.clone(), &node);
            for child in &node.borrow().children {
                unvisited.push_back((
                    child.clone(),
                    VisitationInfo {
                        depth: &info.depth + 1,
                        ..info
                    },
                ));
            }
        } else {
            break;
        }
    }

    state
}

fn parents(node: Rc<RefCell<Tree>>) -> Vec<Rc<RefCell<Tree>>> {
    let mut parents = vec![];
    let mut pointer = node;
    loop {
        if let Some(parent) = &pointer.clone().borrow().parent {
            parents.push(parent.clone());
            pointer = parent.clone();
        } else {
            break;
        }
    }
    parents
}

#[derive(Clone, Debug)]
struct VisitationInfo {
    depth: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tree {
    pub children: Vec<Rc<RefCell<Tree>>>,
    pub parent: Option<Rc<RefCell<Tree>>>,
    pub name: String,
}

impl Tree {
    fn from_edge_strings(strings: &Vec<&str>) -> Result<Vec<Rc<RefCell<Tree>>>, IoError> {
        let edges = strings
            .iter()
            .filter_map(|line| {
                line.split(")")
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .get(0..2)
                    .and_then(|entry| match entry {
                        &[a, b] => Some((a, b)),
                        _ => None,
                    })
            })
            .collect::<Vec<(&str, &str)>>();

        let mut node_lookup = HashMap::new();
        for (from, to) in edges {
            let parent = node_lookup
                .entry(from)
                .or_insert(Rc::new(RefCell::new(Tree {
                    children: vec![],
                    name: from.to_string(),
                    parent: None,
                })))
                .clone();

            let child = node_lookup
                .entry(to)
                .or_insert(Rc::new(RefCell::new(Tree {
                    children: vec![],
                    name: to.to_string(),
                    parent: None,
                })))
                .clone();

            parent.borrow_mut().children.push(child.clone());
            child.borrow_mut().parent = Some(parent.clone());
        }

        find_roots(&node_lookup)
    }
}

fn find_roots(nodes: &HashMap<&str, Rc<RefCell<Tree>>>) -> Result<Vec<Rc<RefCell<Tree>>>, IoError> {
    let mut seen: HashSet<String> = HashSet::new();
    for (&_, n) in nodes {
        let node = n.borrow();
        for child in node.children.iter() {
            seen.insert(child.borrow().name.clone());
        }
    }

    let all = nodes
        .keys()
        .cloned()
        .map(|s| String::from(s))
        .collect::<HashSet<String>>();
    let nodes_without_parent: Vec<Rc<RefCell<Tree>>> = all
        .difference(&seen)
        .cloned()
        .filter_map(|name| nodes.get(name.as_str()))
        .cloned()
        .collect();

    Ok(nodes_without_parent)
}
