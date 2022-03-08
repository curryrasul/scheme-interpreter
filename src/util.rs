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

#[derive(Clone)]
pub struct VariablesSet<T> {
    sets: Vec<NamedArgsList<T>>,
}

impl<T: Clone> VariablesSet<T> {
    pub fn new() -> Self {
        let mut res = VariablesSet { sets: Vec::new() };
        res.sets.push(NamedArgsList::<T>::new());
        return res;
    }

    pub fn find_var(&self, name: &String) -> Option<T> {
        for container in self.sets.iter().rev() {
            let val = container.find_by_name(name);
            if val.is_some() {
                return val;
            }
        }
        return None;
    }

    pub fn add_or_assign_var(&mut self, name: &String, val: T) {
        let cur_set: &mut NamedArgsList<T> = self.sets.last_mut().unwrap();
        for mut var in cur_set.args.iter_mut() {
            if var.0 == *name {
                var.1 = val;
                return;
            }
        }
        cur_set.args.push((name.clone(), val));
    }

    pub fn add_set(&mut self, container: NamedArgsList<T>) {
        self.sets.push(container);
    }

    pub fn pop_set(&mut self) {
        self.sets.pop();
    }
}
