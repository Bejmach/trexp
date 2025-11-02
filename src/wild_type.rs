use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub enum Variant{
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Array(Vec<Variant>, Generic),
    Dictionary(HashMap<String, Variant>, Generic),
    Function(),
    UserType(Rc<RefCell<UserType>>),
    NULL,
}

impl Variant{
    pub fn from_string(str: &str, generic: &Generic) -> Variant{
        match generic {
            &Generic::Any => {
                if str == "true" {
                    return Variant::Bool(true);
                }
                else if str == "false"{
                    return Variant::Bool(false);
                }
                else if str.parse::<i64>().is_ok(){
                    return Variant::Int(str.parse::<i64>().unwrap());
                }
                else if str.parse::<f64>().is_ok(){
                    return Variant::Float(str.parse::<f64>().unwrap());
                }
                else{
                    return Variant::Str(str.to_string());
                }
            },
            &Generic::Int => {
                if str.parse::<i64>().is_ok(){
                    return Variant::Int(str.parse::<i64>().unwrap());
                }
                return Variant::NULL;
            },
            &Generic::Float => {
                if str.parse::<i64>().is_ok(){
                    return Variant::Float(str.parse::<f64>().unwrap());
                }
                return Variant::NULL;
            },
            &Generic::Bool => {
                if str == "true"{
                    return Variant::Bool(true);
                }
                else if str == "false"{
                    return Variant::Bool(false);
                }
                return Variant::NULL;
            },
            &Generic::Str => {
                return Variant::Str(str.to_string());
            },
            _ => {
                return Variant::NULL;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserType{
    pub name: String,
    pub fields: HashMap<String, Variant>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Generic{
    Int,
    Float,
    Bool,
    Str,
    Array(Box<Generic>),
    Object(String),
    Any,
}

impl Generic{
    pub fn from_string(value: &str) -> Self{
        return match value {
            "int" => Generic::Int,
            "float" => Generic::Float,
            "bool" => Generic::Bool,
            "string" => Generic::Str,
            "any" => Generic::Any,
            _ => Generic::Any,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Variable{
    pub variant: Variant,
    pub generic: Generic,
}

pub enum Function{
    Native(fn(Vec<Variable>) -> Variable),
    Defined(DefinedFunction)
}

pub struct DefinedFunction{
    pub params: Vec<String>,
    //Change to nodes, when will be created
    pub body: String
}

pub enum Privacy{
    Public,
    Private,
    Protected,
}
