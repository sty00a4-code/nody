use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Return { None, Return }
pub fn interpret(node: &Node, context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    match node {
        Node::Int { v, pos:_ } => Ok((Some(Value::Int(*v)), Return::None)),
        Node::Float { v, pos:_ } => Ok((Some(Value::Float(*v)), Return::None)),
        Node::Char { v, pos:_ } => Ok((Some(Value::Char(*v)), Return::None)),
        Node::Bool { v, pos:_ } => Ok((Some(Value::Bool(*v)), Return::None)),
        Node::String { v, pos:_ } => Ok((Some(Value::String(v.clone())), Return::None)),
        Node::Vector { nodes, pos:_ } => {
            let mut values: Vec<Value> = vec![];
            let mut typ: Option<Type> = None;
            for n in nodes.iter() {
                let (value, _) = interpret(n, context)?;
                if value.is_none() {
                    return Err(Error::Expected)
                }
                let value = value.unwrap();
                if typ.is_none() {
                    typ = Some(value.typ());
                } else if typ != Some(value.typ()) {
                    return Err(Error::ExpectedType(typ.unwrap(), value.typ()))
                }
                values.push(value);
            }
            Ok((Some(Value::Vector(values, typ)), Return::None))
        }
        Node::Type { v, pos:_ } => Ok((Some(Value::Type(v.clone())), Return::None)),
        Node::Word { v, pos:_ } => match context.get_var(v) {
            Some(v) => Ok((Some(v.clone()), Return::None)),
            None => Err(Error::NotDefined(v.clone()))
        }
        Node::Body { nodes, pos:_ } => {
            context.push();
            for node in nodes.iter() {
                let (value, ret) = interpret(node, context)?;
                if ret != Return::None {
                    context.pop();
                    return Ok((value, ret))
                }
            }
            context.pop();
            Ok((None, Return::None))
        }
        Node::Node { head, args, pos:_ } => {
            let mut values: Vec<Value> = vec![];
            let mut types: Vec<Type> = vec![];
            let mut poses: Vec<Position> = vec![];
            for arg in args.iter() {
                if let (Some(value), _) = interpret(arg, context)? {
                    poses.push(arg.pos().clone());
                    types.push(value.typ());
                    values.push(value);
                } else {
                    return Err(Error::Expected)
                }
            }
            if let Node::Word { v, pos:_ } = head.as_ref() {
                match context.get_native_fn(v, &types) {
                    Some(func) => {
                        let mut func_context = Context::call(context);
                        func_context.create_params(&func.params, values, poses)?;
                        let value = (func.body)(&mut func_context)?;
                        return Ok((value, Return::None))
                    }
                    None => match context.get_fn(v, &types) {
                        Some(func) => {
                            let mut func_context = Context::call(context);
                            func_context.create_params(&func.params, values, poses)?;
                            let (value, _) = interpret(&func.body, &mut func_context)?;
                            return Ok((value, Return::None))
                        }
                        None => match context.get_var(v) {
                            Some(_) => {}
                            None => if context.fn_exists(v) || context.native_fn_exists(v) {
                                return Err(Error::FunctionPatternNotFound(v.clone(), types))
                            } else {
                                return Err(Error::NotDefined(v.clone()))
                            }
                        }
                    }
                }
            }
            if let Some(head_value) = interpret(head, context)?.0 {
                if types.len() == 0 { return Ok((Some(head_value), Return::None)) }
                match head_value {
                    Value::Type(typ) => match context.get_native_fn(&typ.to_string(), &types) {
                        Some(func) => {
                            let mut func_context = Context::call(context);
                            func_context.create_params(&func.params, values, poses)?;
                            let value = (func.body)(&mut func_context)?;
                            return Ok((value, Return::None))
                        }
                        None => if context.fn_exists(&typ.to_string()) || context.native_fn_exists(&typ.to_string()) {
                            Err(Error::InvalidCastBetween(typ.clone(), types[0].clone()))
                        } else {
                            Err(Error::InvalidHeadCastType(typ.clone()))
                        }
                    }
                    Value::Vector(vec_values, typ) => if values.len() == 1 {
                        match &values[0] {
                            Value::Int(idx) => if *idx < 0 {
                                match vec_values.get(vec_values.len() - idx.abs() as usize) {
                                    Some(value) => Ok((Some(value.clone()), Return::None)),
                                    None => Err(Error::IndexOutOfRange(vec_values.len() - idx.abs() as usize, vec_values.len()))
                                }
                            } else {
                                match vec_values.get(*idx as usize) {
                                    Some(value) => Ok((Some(value.clone()), Return::None)),
                                    None => Err(Error::IndexOutOfRange(*idx as usize, vec_values.len()))
                                }
                            }
                            _ => Err(Error::ExpectedTypes(vec![Type::Int], types[0].clone()))
                        }
                    } else {
                        todo!()
                    }
                    _ => Err(Error::InvalidHeadValue(head_value.clone()))
                }
            } else {
                Err(Error::Expected)
            }
        }
        _ => todo!("{node}")
    }
}