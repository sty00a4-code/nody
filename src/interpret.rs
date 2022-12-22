use crate::*;

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, (Value, bool, Position)>,
    funcs: HashMap<String, Vec<(Function, Position)>>,
    subs: HashMap<String, Scope>
}
impl Scope {
    pub fn new() -> Self { Scope { vars: HashMap::new(), funcs: HashMap::new(), subs: HashMap::new() } }
    pub fn create_var(&mut self, id: String, value: Value, mutable: bool, pos: Position) -> Result<(), Error> {
        if self.vars.contains_key(&id) { todo!("already defined") }
        self.vars.insert(id, (value, mutable, pos));
        Ok(())
    }
    pub fn del_var(&mut self, id: &String) -> Option<(Value, bool, Position)> {
        self.vars.remove(id)
    }
    pub fn change(&mut self, id: String, value: Value) -> Result<(), Error> {
        match self.vars.get_mut(&id) {
            Some((old_value, mutable, pos)) => if *mutable {
                *old_value = value;
                Ok(())
            } else {
                todo!("immutable")
            }
            None => todo!("not defined")
        }
    }
    pub fn get_var(&self, id: &String) -> Option<&Value> {
        match self.vars.get(id) {
            Some((value, _, _)) => Some(value),
            None => None
        }
    }
    pub fn is_mutable(&self, id: &String) -> Option<bool> {
        match self.vars.get(id) {
            Some((_, mutable, _)) => Some(*mutable),
            None => None
        }
    }
    pub fn get_var_mut(&mut self, id: &String) -> Option<&mut Value> {
        match self.vars.get_mut(id) {
            Some((value, _, _)) => Some(value),
            None => None
        }
    }
    pub fn get_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Function> {
        match self.funcs.get(id) {
            Some(defs) => todo!("find function with pattern"),
            None => None
        }
    }
    pub fn get_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Function> {
        match self.funcs.get_mut(id) {
            Some(defs) => todo!("find function with pattern"),
            None => None
        }
    }
}
#[derive(Debug, Clone)]
pub struct Context {
    pub scopes: Vec<Scope>,
    pub global: Scope,
    pub trace: Vec<Position>
}
impl Context {
    pub fn new() -> Self { Self { scopes: vec![Scope::new()], global: Scope::new(), trace: vec![] } }
    pub fn push(&mut self) { self.scopes.push(Scope::new()) }
    pub fn pop(&mut self) -> Option<Scope> { self.scopes.pop() }
    pub fn trace_push(&mut self, pos: &Position) { self.trace.push(pos.clone()); }
    pub fn trace_pop(&mut self) -> Option<Position> { self.trace.pop() }
    pub fn get_scope_var(&self, id: &String) -> Option<&Scope> {
        for scope in self.scopes.iter() {
            if scope.get_var(id).is_some() { return Some(scope) }
        }
        if self.global.get_var(id).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_var_mut(&mut self, id: &String) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut() {
            if scope.get_var(id).is_some() { return Some(scope) }
        }
        if self.global.get_var(id).is_some() { return Some(&mut self.global) }
        None
    }
    pub fn get_scope_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Scope> {
        for scope in self.scopes.iter() {
            if scope.get_fn(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_fn(id, pattern).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut() {
            if scope.get_fn_mut(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_fn_mut(id, pattern).is_some() { return Some(&mut self.global) }
        None
    }
    pub fn create_var(&mut self, id: String, value: Value, mutable: bool, pos: Position) -> Result<(), Error> {
        match self.get_scope_var_mut(&id) {
            None => self.scopes.last_mut().unwrap().create_var(id, value, mutable, pos),
            Some(_) => todo!("already defined")
        }
    }
    pub fn del_var(&mut self, id: &String) -> Option<(Value, bool, Position)> {
        self.get_scope_var_mut(&id)?.del_var(id)
    }
    pub fn change(&mut self, id: String, value: Value) -> Result<(), Error> {
        match self.global.vars.get_mut(&id) {
            Some((old_value, mutable, pos)) => if *mutable {
                *old_value = value;
                return Ok(())
            } else {
                todo!("immutable")
            }
            None => {}
        }
        for scope in self.scopes.iter_mut().rev() {
            match scope.vars.get_mut(&id) {
                Some((old_value, mutable, pos)) => if *mutable {
                    *old_value = value;
                    return Ok(())
                } else {
                    todo!("immutable")
                }
                None => {}
            }
        }
        todo!("not defined")
    }
    pub fn get_var(&self, id: &String) -> Option<&Value> {
        match self.get_scope_var(id) {
            Some(scope) => scope.get_var(id),
            None => self.global.get_var(id)
        }
    }
    pub fn is_mutable(&self, id: &String) -> Option<bool> {
        match self.get_scope_var(id) {
            Some(scope) => scope.is_mutable(id),
            None => self.global.is_mutable(id)
        }
    }
    pub fn get_var_mut(&mut self, id: &String) -> Option<&mut Value> {
        for scope in self.scopes.iter_mut() {
            match scope.get_var_mut(id) {
                Some(value) => return Some(value),
                None => {}
            }
        }
        match self.global.get_var_mut(id) {
            Some(value) => Some(value),
            None => None
        }
    }
    pub fn get_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Function> {
        self.get_scope_fn(id, pattern)?.get_fn(id, pattern)
    }
    pub fn get_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Function> {
        self.get_scope_fn_mut(id, pattern)?.get_fn_mut(id, pattern)
    }
}