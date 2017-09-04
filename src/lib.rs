#![feature(box_patterns)]

#[macro_use]
extern crate error_chain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

mod error {
    error_chain! {
        errors {
            NotAFunction(t: ::Form) {
                description("Tried to call something that is not a function")
                display("{:?} is not a function", t)
            }
            IdentifierNotFound(id: String) {
                description("Tried to access a non existent value")
                display("{} doesn't exist", id)
            }
        }
    }
}
use error::*;

#[derive(Debug, Clone, PartialEq)]
enum Form {
    Int(i64),
    Float(f64),
    Str(String),
    Ident(String),
    Form(ConsList),
    Quote(ConsList),
    Func(Function),
}

#[derive(Debug, Clone, PartialEq)]
enum ConsList {
    Nil,
    Cons(Box<(Form, ConsList)>),
}


impl Form {
    pub fn eval(&self, env: Environment) -> Result<Form> {
        use Form::*;
        use ConsList::*;
        Ok(match self.clone() {
            i @ Int(_) => i,
            f @ Float(_) => f,
            s @ Str(_) => s,
            q @ Quote(_) => q,
            f @ Form(Nil) => f,
            f @ Func(_) => f,
            Ident(ident) => env.get(ident)?,
            Form(Cons(box (Func(f), Cons(box (head, _))))) => f.call(head),
            Form(Cons(box (head, _))) => bail!(ErrorKind::NotAFunction(head)),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Function;

impl Function {
    pub fn call(&self, input: Form) -> Form {
        let env = Environment::Cons {
            rest: Box::new(Environment::Nil),
            val: input,
            ident: unimplemented!(), // TODO: The identifier of the current currying level,
                                     // needs to be taken from the Parameter list
                                     // which is eventually contained in `Function`
        };
    }
}

#[derive(Debug, Clone)]
enum Environment {
    Cons {
        rest: Box<Environment>,
        val: Form,
        ident: String,
    },
    Nil,
}

impl Environment {
    pub fn get(&self, id: String) -> Result<Form> {
        match self {
            &Environment::Cons {
                ref rest,
                ref val,
                ref ident,
            } => {
                if &id == ident {
                    Ok(val.clone())
                } else {
                    rest.get(id)
                }
            }
            &Environment::Nil => bail!(ErrorKind::IdentifierNotFound(id)),
        }
    }
}

struct TypedCons<H, T: ConsElem> {
    head: H,
    tail: T,
}

struct TypedNil;

trait ConsElem {}

impl<H, T: ConsElem> ConsElem for TypedCons<H, T> {}
impl ConsElem for TypedNil {}
