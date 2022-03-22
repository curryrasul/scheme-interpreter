#[derive(Clone)]
pub struct NamedArgsList<T> {
    args: Vec<(String, T)>,
}

impl<T: Clone> NamedArgsList<T> {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    pub fn get_ith_val(&self, ind: usize) -> T {
        self.args[ind].1.clone()
    }

    pub fn find_by_name(&self, name: &str) -> Option<T> {
        for arg in self.args.iter() {
            if arg.0 == *name {
                return Some(arg.1.clone());
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (String, T)> {
        self.args.iter()
    }

    pub fn copy_values(&self) -> Vec<T> {
        let mut res = Vec::new();
        for arg in self.args.iter() {
            res.push(arg.1.clone());
        }
        res
    }
}

impl<T: Clone> Default for NamedArgsList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct VariablesSet<T> {
    sets: Vec<NamedArgsList<T>>,
}

impl<T: Clone> VariablesSet<T> {
    pub fn new() -> Self {
        let mut res = VariablesSet { sets: Vec::new() };
        res.sets.push(NamedArgsList::<T>::new());
        res
    }

    pub fn find_var(&self, name: &str) -> Option<T> {
        for container in self.sets.iter().rev() {
            let val = container.find_by_name(name);
            if val.is_some() {
                return val;
            }
        }
        None
    }

    pub fn add_or_assign_var(&mut self, name: &str, val: T) {
        let cur_set: &mut NamedArgsList<T> = self.sets.last_mut().unwrap();
        for mut var in cur_set.args.iter_mut() {
            if var.0 == *name {
                var.1 = val;
                return;
            }
        }
        cur_set.args.push((String::from(name), val));
    }

    pub fn add_set(&mut self, container: NamedArgsList<T>) {
        self.sets.push(container);
    }

    pub fn pop_set(&mut self) {
        self.sets.pop();
    }
}

impl<T: Clone> Default for VariablesSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
