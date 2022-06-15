use std::{
    borrow::Cow,
};


pub type Block<'input>=Vec<Statement<'input>>;


#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Visibility {
    Library,
    Local,
    Full,
    None,
}
impl From<Option<Self>> for Visibility {
    fn from(opt:Option<Self>)->Self {
        opt.unwrap_or_default()
    }
}
impl Default for Visibility {
    fn default()->Self {
        Visibility::None
    }
}
#[derive(Debug)]
pub enum Type<'input> {
    Named {
        name:&'input str,
        generics:Vec<Self>,
    },
    Object {
        fields:Vec<TypeObjectField<'input>>,
        exact:bool,
    },
    Union(Vec<Self>),
    Composite(Vec<Self>),
    FunctionSig(Box<AnonFunctionSignature<'input>>),
    Uint,
    Int,
    Float,
    DoubleFloat,
    Byte,
    Char,
    String,
    GenericNumber,
    GenericFloat,
    Unknown,
}
#[derive(Debug)]
pub enum Statement<'input> {
    FunctionDef(Function<'input>),
    FunctionSig(FunctionSignature<'input>),
    InterfaceDef(Interface<'input>),
    TypeDef(TypeDef<'input>),
    VarDef(VarDef<'input>),
    VarAssign(VarAssign<'input>),
    Expr(Expr<'input>),
    Doc(&'input str),
}
#[derive(Debug)]
pub enum Expr<'input> {
    FieldAccess {
        from:Box<Self>,
        name:&'input str,
    },
    MethodCall {
        from:Box<Self>,
        name:&'input str,
        args:Vec<Self>,
    },
    FunctionCall {
        name:&'input str,
        args:Vec<Self>,
    },
    AssociatedPath(Vec<&'input str>),
    Var(&'input str),
    Block(Block<'input>),
    Data(Data<'input>),
    Add(Box<[Self;2]>),
    Sub(Box<[Self;2]>),
    Mul(Box<[Self;2]>),
    Div(Box<[Self;2]>),
    Mod(Box<[Self;2]>),
    Negate(Box<Self>),
    Equal(Box<[Self;2]>),
    NotEqual(Box<[Self;2]>),
    GreaterEqual(Box<[Self;2]>),
    LessEqual(Box<[Self;2]>),
    Greater(Box<[Self;2]>),
    Less(Box<[Self;2]>),
    And(Box<[Self;2]>),
    Or(Box<[Self;2]>),
    Not(Box<Self>),
    ObjectCreation(Vec<(&'input str,Self)>),
    AnonFunction(AnonFunction<'input>),
}
#[derive(Debug)]
pub enum Data<'input> {
    String(Cow<'input,str>),
    GenericNumber(&'input str),
    GenericFloat(&'input str),
    UInt(u64),
    Int(i64),
    Float(f32),
    LargeFloat(f64),
    Char(char),
}
#[derive(Debug)]
pub enum MethodType {
    This,
    ThisMut,
    None,
}


#[derive(Debug)]
pub struct AnonFunctionSignature<'input> {
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
}
impl<'input> AnonFunctionSignature<'input> {
    pub fn to_function(self,block:Block<'input>)->AnonFunction<'input> {
        let AnonFunctionSignature{params,ret_type}=self;
        AnonFunction {
            params,
            ret_type,
            block,
        }
    }
}
#[derive(Debug)]
pub struct AnonFunction<'input> {
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct FunctionSignature<'input> {
    pub public:Visibility,
    pub name:&'input str,
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
}
impl<'input> FunctionSignature<'input> {
    pub fn to_function(self,block:Block<'input>)->Function<'input> {
        let FunctionSignature{public,name,params,ret_type}=self;
        Function {
            public,
            name,
            params,
            ret_type,
            block,
        }
    }
}
#[derive(Debug)]
pub struct Function<'input> {
    pub public:Visibility,
    pub name:&'input str,
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct Parameters<'input> {
    pub method_type:MethodType,
    pub normal:Vec<Parameter<'input>>,
    pub var_arg:Option<Parameter<'input>>,
}
impl<'input> Default for Parameters<'input> {
    fn default()->Self {
        Parameters {
            method_type:MethodType::None,
            normal:Vec::new(),
            var_arg:None,
        }
    }
}
#[derive(Debug)]
pub struct Parameter<'input> {
    pub mutable:bool,
    pub name:&'input str,
    pub ty:Type<'input>,
}
#[derive(Debug)]
pub struct TypeObjectField<'input> {
    pub public:Visibility,
    pub mutable:Visibility,
    pub name:&'input str,
    pub ty:Type<'input>,
    pub doc:Option<&'input str>,
}
#[derive(Debug)]
pub struct Interface<'input> {
    pub public:Visibility,
    pub name:&'input str,
    pub requirement:Option<Type<'input>>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct TypeDef<'input> {
    pub public:Visibility,
    pub name:&'input str,
    pub ty:Type<'input>,
}
#[derive(Debug)]
pub struct VarDef<'input> {
    pub public:Visibility,
    pub mutable:Visibility,
    pub name:&'input str,
    pub ty:Option<Type<'input>>,
    pub data:Option<Expr<'input>>,
}
#[derive(Debug)]
pub struct VarAssign<'input> {
    pub name:&'input str,
    pub data:Expr<'input>,
}
