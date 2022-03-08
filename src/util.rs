#[derive(Clone)]
pub struct NamedArgsList<T> {
    args: Vec<(String, T)>,
}

impl<T: Clone> NamedArgsList<T> {
    pub fn new() -> Self {
        return Self { args: Vec::new() };
    }

    pub fn get_ith_val(&self, ind: usize) -> T {
        return self.args[ind].1.clone();
    }

    pub fn find_by_name(&self, name: &String) -> Option<T> {
        for arg in self.args.iter() {
            if arg.0 == *name {
                return Some(arg.1.clone());
            }
        }
        return None;
    }

    pub fn len(&self) -> usize {
        return self.args.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (String, T)> {
        return self.args.iter();
    }

    pub fn copy_values(&self) -> Vec<T> {
        let mut res = Vec::new();
        for arg in self.args.iter() {
            res.push(arg.1.clone());
        }
        return res;
    }
}
