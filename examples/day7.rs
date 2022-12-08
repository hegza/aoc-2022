use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day7.txt");

#[derive(Hash, PartialEq, Eq, Clone)]
struct Path(Vec<String>);

impl Path {
    fn root() -> Self {
        Self(vec![])
    }

    fn parent(&self) -> Self {
        Self(self.0[0..self.0.len() - 1].to_vec())
    }

    fn join(&self, dir: impl Into<String>) -> Self {
        let mut child = self.0.clone();
        child.push(dir.into());
        Self(child)
    }

    fn to_string(&self) -> String {
        let mut path = "/".to_string();
        path.push_str(&self.0.join("/"));
        path
    }

    fn cd(&mut self, param: &str) {
        match param {
            "/" => *self = Path::root(),
            ".." => {
                *self = self.parent();
            }
            dir => {
                *self = self.join(dir);
            }
        }
    }

    fn len(&self) -> usize {
        // Root counts for one here
        self.0.len() + 1
    }
}

#[derive(Eq, PartialEq)]
struct DDir(Path, HashMap<Path, DTree>);

impl std::hash::Hash for DDir {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl DDir {
    fn insert_dir(&mut self, path: Path) {
        let set = &mut self.1;

        // Path is longer by one element -> is child of this
        if path.len() == self.0.len() + 1 {
            set.insert(path.clone(), DTree::Dir(DDir(path.clone(), HashMap::new())));
            println!(
                "Inserted dir {} at {}",
                path.to_string(),
                self.0.to_string()
            );
            return;
        }

        // There is more to go
        let next_path = Path(path.0[0..self.0.len()].to_vec());
        if !set.contains_key(&next_path) {
            set.insert(
                next_path.clone(),
                DTree::Dir(DDir(next_path.clone(), HashMap::new())),
            );
        }

        set.entry(next_path)
            .and_modify(|dtree| dtree.insert_dir(path));
    }

    fn insert_file(&mut self, path: Path, fsize: usize) {
        let set = &mut self.1;

        // Path is longer by one element -> is child of this
        if path.len() == self.0.len() + 1 {
            set.insert(path.clone(), DTree::File(path.clone(), fsize));
            println!(
                "Inserted file {} at {}",
                path.to_string(),
                self.0.to_string()
            );
            return;
        }

        // There is more to go
        let next_path = Path(path.0[0..self.0.len()].to_vec());
        if !set.contains_key(&next_path) {
            set.insert(
                next_path.clone(),
                DTree::Dir(DDir(next_path.clone(), HashMap::new())),
            );
        }

        set.entry(next_path)
            .and_modify(|dtree| dtree.insert_file(path, fsize));
    }

    fn dirs(&self) -> Vec<&DDir> {
        let mut v = vec![];
        for d in self.1.values() {
            match d {
                DTree::Dir(ddir) => {
                    v.push(ddir);
                    v.extend(ddir.dirs());
                }
                _ => {}
            }
        }
        v
    }

    fn size(&self) -> usize {
        let mut acc = 0;
        for d in self.1.values() {
            match d {
                DTree::Dir(d) => {
                    acc += d.size();
                }
                DTree::File(_, fsize) => {
                    acc += fsize;
                }
            }
        }
        acc
    }
}

#[derive(Eq, Hash, PartialEq)]
enum DTree {
    Dir(DDir),
    File(Path, usize),
}

impl DTree {
    fn insert_dir(&mut self, path: Path) {
        match self {
            DTree::Dir(dir) => {
                dir.insert_dir(path);
            }
            DTree::File(_, _) => panic!("cannot insert dir into file"),
        }
    }

    fn insert_file(&mut self, path: Path, fsize: usize) {
        match self {
            DTree::Dir(dir) => {
                dir.insert_file(path, fsize);
            }
            DTree::File(_, _) => panic!("cannot insert file into file"),
        }
    }

    fn dirs(&self) -> Vec<&DDir> {
        let mut v = vec![];
        match self {
            DTree::Dir(ddir) => {
                v.push(ddir);
                v.extend(ddir.dirs());
            }
            DTree::File(_, _) => {}
        }
        v
    }

    fn size(&self) -> usize {
        match self {
            DTree::Dir(d) => d.size(),
            DTree::File(_, fsize) => *fsize,
        }
    }

    /*
    fn print(depth: usize) -> ! {
        match self {
            DTree::Dir(d) => {},
            DTree::File(fname, _size) => {}
        }
    }
    */
}

fn main() -> anyhow::Result<()> {
    let mut cur_path = Path::root();
    let mut dtree = DTree::Dir(DDir(Path::root(), HashMap::new()));

    let mut lines = INPUT.lines();
    while let Some(next_line) = lines.next() {
        let mut toks = next_line.split_ascii_whitespace();

        let first_tok = toks.next().unwrap();

        if first_tok == "$" {
            let cmd = toks.next().unwrap();
            match cmd {
                "cd" => {
                    let cd_path = toks.next().unwrap();
                    cur_path.cd(cd_path);
                    println!("cd {} -> {}", cd_path, cur_path.to_string())
                }
                "ls" => {}
                _ => panic!("unknown cmd: {}", cmd),
            }
        }
        // This is output, because it's not a command ("$")
        else {
            match first_tok {
                "dir" => {
                    let dname = toks.next().unwrap();
                    dtree.insert_dir(cur_path.join(dname));
                }
                fsize => {
                    let fsize = fsize.parse::<usize>()?;
                    let fname = toks.next().unwrap();
                    dtree.insert_file(cur_path.join(fname), fsize);
                }
            }
        }
    }

    let sizes: usize = dtree
        .dirs()
        .iter()
        .filter(|dir| dir.size() <= 100000)
        .map(|dir| dir.size())
        .sum();
    println!("Part 1: {}", sizes);

    let total = 70000000;
    let required = 30000000;
    let currenet = dtree.size();
    let remaining = total - currenet;
    let need_to_free = required - remaining;

    let smallest_freeable = dtree
        .dirs()
        .iter()
        .filter(|dir| dir.size() >= need_to_free)
        .map(|dir| dir.size())
        .min()
        .unwrap();

    println!("Part 2: {}", smallest_freeable);

    Ok(())
}
