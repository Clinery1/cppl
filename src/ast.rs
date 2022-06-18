use std::{
    borrow::Cow,
};


pub type Block<'input>=Vec<Statement<'input>>;


#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Visibility {
    Library,
    Local,
    Full,
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
    Bool,
    Char,
    String,
    Never,
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
    StaticVarDef(StaticVarDef<'input>),
    ConstVarDef(ConstVarDef<'input>),
    VarAssign(VarAssign<'input>),
    Expr(Expr<'input>),
    Import(Import<'input>),
    Return {
        label:Option<&'input str>,
        val:Option<Expr<'input>>,
    },
    Continue(Option<&'input str>),
    Enum(Enum<'input>),
    Module(&'input str),
    Impl(Impl<'input>),
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
    FunctionCall(FunctionCall<'input>),
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
    IsType(Box<Self>,Type<'input>),
    ObjectCreation(Vec<ObjectField<'input>>),
    AnonFunction(AnonFunction<'input>),
    Ref(Box<Self>),
    RefMut(Box<Self>),
    ForeverLoop(Vec<Statement<'input>>),
    WhileLoop {
        condition:Box<Expr<'input>>,
        block:Vec<Statement<'input>>,
    },
    ForLoop {
        var:&'input str,
        iterator:Box<Expr<'input>>,
        block:Vec<Statement<'input>>,
    },
    Match(Box<Match<'input>>),
}
#[derive(Debug)]
pub enum Data<'input> {
    String(Cow<'input,str>),
    GenericNumber(bool,&'input str),
    GenericFloat(bool,&'input str),
    UInt(u64),
    Int(i64),
    Float(f32),
    LargeFloat(f64),
    Char(char),
    Bool(bool),
}
#[derive(Debug)]
pub enum MethodType {
    This,
    ThisMut,
    None,
}
#[derive(Debug)]
pub enum Import<'input> {
    Path(Vec<&'input str>),
    PathBlock {
        path:Vec<&'input str>,
        block:Vec<Self>,
    },
}
#[derive(Debug)]
pub enum MatchPattern<'input> {
    Data(Data<'input>),
    MethodCall {
        name:&'input str,
        args:Vec<Expr<'input>>,
    },
    Structure(MatchPatternStructure<'input>),
    Var(&'input str),
    Equal(Expr<'input>),
    NotEqual(Expr<'input>),
    GreaterEqual(Expr<'input>),
    LessEqual(Expr<'input>),
    Greater(Expr<'input>),
    Less(Expr<'input>),
    IsType(Type<'input>),
}
#[derive(Debug)]
pub enum MatchPatternStructureItem<'input> {
    Field(&'input str),
    NamedField {
        name:&'input str,
        rename:&'input str,
    },
    NamedBlock {
        name:&'input str,
        block:MatchPatternStructure<'input>,
    },
}
#[derive(Debug)]
pub enum MatchPatternStructure<'input> {
    Block {
        exact:bool,
        block:Vec<MatchPatternStructureItem<'input>>,
    },
    TypedBlock {
        exact:bool,
        type_name:&'input str,
        block:Vec<MatchPatternStructureItem<'input>>,
    },
}


#[derive(Debug)]
pub struct Impl<'input> {
    pub params:Option<Vec<TypeParameter<'input>>>,
    pub interface:Option<Type<'input>>,
    pub for_ty:Type<'input>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct Enum<'input> {
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Option<Vec<TypeParameter<'input>>>,
    pub variants:Vec<Type<'input>>,
}
#[derive(Debug)]
pub struct FunctionCall<'input> {
    pub path:Vec<&'input str>,
    pub args:Vec<Expr<'input>>,
}
#[derive(Debug)]
pub struct Match<'input> {
    pub to_match:Expr<'input>,
    pub leafs:Vec<(MatchPattern<'input>,Expr<'input>)>,
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
    pub public:Option<Visibility>,
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
    pub public:Option<Visibility>,
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
    pub public:Option<Visibility>,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub ty:Type<'input>,
}
#[derive(Debug)]
pub struct Interface<'input> {
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Option<Vec<TypeParameter<'input>>>,
    pub requirement:Option<Type<'input>>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct TypeDef<'input> {
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Option<Vec<TypeParameter<'input>>>,
    pub ty:Type<'input>,
}
#[derive(Debug)]
pub struct TypeParameter<'input> {
    pub name:&'input str,
    pub ty:Option<Type<'input>>,
}
#[derive(Debug)]
pub struct VarDef<'input> {
    pub public:Option<Visibility>,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub ty:Option<Type<'input>>,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct StaticVarDef<'input> {
    pub public:Option<Visibility>,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub ty:Type<'input>,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct ConstVarDef<'input> {
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub ty:Type<'input>,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct VarAssign<'input> {
    pub name:&'input str,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct ObjectField<'input> {
    pub public:Option<Visibility>,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub data:Expr<'input>,
}
