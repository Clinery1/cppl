use std::{
    borrow::Cow,
};
use cppl_error::Location;


#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Visibility {
    Library{start:Location,end:Location},
    Local{start:Location,end:Location},
    Full{start:Location,end:Location},
}
#[derive(Debug)]
pub enum Type<'input> {
    Named {
        start:Location,
        end:Location,
        name:&'input str,
        generics:Vec<Self>,
    },
    Object {
        start:Location,
        end:Location,
        fields:Vec<TypeObjectField<'input>>,
        exact:bool,
    },
    Union {
        start:Location,
        end:Location,
        inner:Vec<Self>,
    },
    Composite {
        start:Location,
        end:Location,
        inner:Vec<Self>,
    },
    FunctionSig {
        start:Location,
        end:Location,
        inner:Box<AnonFunctionSignature<'input>>,
    },
    Uint{start:Location,end:Location},
    Int{start:Location,end:Location},
    Float{start:Location,end:Location},
    DoubleFloat{start:Location,end:Location},
    Byte{start:Location,end:Location},
    Bool{start:Location,end:Location},
    Char{start:Location,end:Location},
    String{start:Location,end:Location},
    Never{start:Location,end:Location},
    GenericNumber{start:Location,end:Location},
    GenericFloat{start:Location,end:Location},
}
impl<'input> Type<'input> {
    pub fn start(&self)->Location {
        use Type::*;
        *match self {
            Named{start,..}=>start,
            Object{start,..}=>start,
            Union{start,..}=>start,
            Composite{start,..}=>start,
            FunctionSig{start,..}=>start,
            Uint{start,..}=>start,
            Int{start,..}=>start,
            Float{start,..}=>start,
            DoubleFloat{start,..}=>start,
            Byte{start,..}=>start,
            Bool{start,..}=>start,
            Char{start,..}=>start,
            String{start,..}=>start,
            Never{start,..}=>start,
            GenericNumber{start,..}=>start,
            GenericFloat{start,..}=>start,
        }
    }
    pub fn end(&self)->Location {
        use Type::*;
        *match self {
            Named{end,..}=>end,
            Object{end,..}=>end,
            Union{end,..}=>end,
            Composite{end,..}=>end,
            FunctionSig{end,..}=>end,
            Uint{end,..}=>end,
            Int{end,..}=>end,
            Float{end,..}=>end,
            DoubleFloat{end,..}=>end,
            Byte{end,..}=>end,
            Bool{end,..}=>end,
            Char{end,..}=>end,
            String{end,..}=>end,
            Never{end,..}=>end,
            GenericNumber{end,..}=>end,
            GenericFloat{end,..}=>end,
        }
    }
}
#[derive(Debug)]
pub enum Statement<'input> {
    FunctionDef{start:Location,end:Location,inner:Function<'input>},
    FunctionSig{start:Location,end:Location,inner:FunctionSignature<'input>},
    InterfaceDef{start:Location,end:Location,inner:Interface<'input>},
    TypeDef{start:Location,end:Location,inner:TypeDef<'input>},
    VarDef{start:Location,end:Location,inner:VarDef<'input>},
    StaticVarDef{start:Location,end:Location,inner:StaticVarDef<'input>},
    ConstVarDef{start:Location,end:Location,inner:ConstVarDef<'input>},
    VarAssign{start:Location,end:Location,inner:VarAssign<'input>},
    Expr{start:Location,end:Location,inner:Expr<'input>},
    Import{start:Location,end:Location,inner:Import<'input>},
    Return {
        start:Location,
        end:Location,
        label:Option<&'input str>,
        val:Option<Expr<'input>>,
    },
    Continue{start:Location,end:Location,inner:Option<&'input str>},
    Enum{start:Location,end:Location,inner:Enum<'input>},
    Module{start:Location,end:Location,inner:&'input str},
    Impl{start:Location,end:Location,inner:Impl<'input>},
}
impl<'input> Statement<'input> {
    pub fn start(&self)->Location {
        use Statement::*;
        *match self {
            FunctionDef{start,..}=>start,
            FunctionSig{start,..}=>start,
            InterfaceDef{start,..}=>start,
            TypeDef{start,..}=>start,
            VarDef{start,..}=>start,
            StaticVarDef{start,..}=>start,
            ConstVarDef{start,..}=>start,
            VarAssign{start,..}=>start,
            Expr{start,..}=>start,
            Import{start,..}=>start,
            Return{start,..}=>start,
            Continue{start,..}=>start,
            Enum{start,..}=>start,
            Module{start,..}=>start,
            Impl{start,..}=>start,
        }
    }
    pub fn end(&self)->Location {
        use Statement::*;
        *match self {
            FunctionDef{end,..}=>end,
            FunctionSig{end,..}=>end,
            InterfaceDef{end,..}=>end,
            TypeDef{end,..}=>end,
            VarDef{end,..}=>end,
            StaticVarDef{end,..}=>end,
            ConstVarDef{end,..}=>end,
            VarAssign{end,..}=>end,
            Expr{end,..}=>end,
            Import{end,..}=>end,
            Return{end,..}=>end,
            Continue{end,..}=>end,
            Enum{end,..}=>end,
            Module{end,..}=>end,
            Impl{end,..}=>end,
        }
    }
}
#[derive(Debug)]
pub enum Expr<'input> {
    FieldAccess {
        start:Location,
        end:Location,
        from:Box<Self>,
        name:&'input str,
    },
    MethodCall {
        start:Location,
        end:Location,
        from:Box<Self>,
        name:&'input str,
        args:Vec<Self>,
    },
    FunctionCall{
        start:Location,
        end:Location,
        path:Vec<&'input str>,
        args:Vec<Self>,
    },
    AssociatedPath{start:Location,end:Location,inner:Vec<&'input str>},
    Var{start:Location,end:Location,inner:&'input str},
    Block{start:Location,end:Location,inner:Block<'input>},
    Data{start:Location,end:Location,inner:Data<'input>},
    Add{start:Location,end:Location,inner:Box<[Self;2]>},
    Sub{start:Location,end:Location,inner:Box<[Self;2]>},
    Mul{start:Location,end:Location,inner:Box<[Self;2]>},
    Div{start:Location,end:Location,inner:Box<[Self;2]>},
    Mod{start:Location,end:Location,inner:Box<[Self;2]>},
    Negate{start:Location,end:Location,inner:Box<Self>},
    Equal{start:Location,end:Location,inner:Box<[Self;2]>},
    NotEqual{start:Location,end:Location,inner:Box<[Self;2]>},
    GreaterEqual{start:Location,end:Location,inner:Box<[Self;2]>},
    LessEqual{start:Location,end:Location,inner:Box<[Self;2]>},
    Greater{start:Location,end:Location,inner:Box<[Self;2]>},
    Less{start:Location,end:Location,inner:Box<[Self;2]>},
    And{start:Location,end:Location,inner:Box<[Self;2]>},
    Or{start:Location,end:Location,inner:Box<[Self;2]>},
    Not{start:Location,end:Location,inner:Box<Self>},
    IsType{start:Location,end:Location,inner:Box<Self>,ty:Type<'input>},
    ObjectCreation{start:Location,end:Location,inner:Vec<ObjectField<'input>>},
    AnonFunction{start:Location,end:Location,inner:AnonFunction<'input>},
    Ref{start:Location,end:Location,inner:Box<Self>},
    RefMut{start:Location,end:Location,inner:Box<Self>},
    ForeverLoop{start:Location,end:Location,inner:Block<'input>},
    WhileLoop {
        start:Location,
        end:Location,
        condition:Box<Self>,
        block:Block<'input>,
    },
    ForLoop {
        start:Location,
        end:Location,
        var:&'input str,
        iterator:Box<Self>,
        block:Block<'input>,
    },
    Match{start:Location,end:Location,inner:Box<Match<'input>>},
}
impl<'input> Expr<'input> {
    pub fn start(&self)->Location {
        use Expr::*;
        *match self {
            FieldAccess{start,..}=>start,
            MethodCall{start,..}=>start,
            FunctionCall{start,..}=>start,
            AssociatedPath{start,..}=>start,
            Var{start,..}=>start,
            Block{start,..}=>start,
            Data{start,..}=>start,
            Add{start,..}=>start,
            Sub{start,..}=>start,
            Mul{start,..}=>start,
            Div{start,..}=>start,
            Mod{start,..}=>start,
            Negate{start,..}=>start,
            Equal{start,..}=>start,
            NotEqual{start,..}=>start,
            GreaterEqual{start,..}=>start,
            LessEqual{start,..}=>start,
            Greater{start,..}=>start,
            Less{start,..}=>start,
            And{start,..}=>start,
            Or{start,..}=>start,
            Not{start,..}=>start,
            IsType{start,..}=>start,
            ObjectCreation{start,..}=>start,
            AnonFunction{start,..}=>start,
            Ref{start,..}=>start,
            RefMut{start,..}=>start,
            ForeverLoop{start,..}=>start,
            WhileLoop{start,..}=>start,
            ForLoop{start,..}=>start,
            Match{start,..}=>start,
        }
    }
    pub fn end(&self)->Location {
        use Expr::*;
        *match self {
            FieldAccess{end,..}=>end,
            MethodCall{end,..}=>end,
            FunctionCall{end,..}=>end,
            AssociatedPath{end,..}=>end,
            Var{end,..}=>end,
            Block{end,..}=>end,
            Data{end,..}=>end,
            Add{end,..}=>end,
            Sub{end,..}=>end,
            Mul{end,..}=>end,
            Div{end,..}=>end,
            Mod{end,..}=>end,
            Negate{end,..}=>end,
            Equal{end,..}=>end,
            NotEqual{end,..}=>end,
            GreaterEqual{end,..}=>end,
            LessEqual{end,..}=>end,
            Greater{end,..}=>end,
            Less{end,..}=>end,
            And{end,..}=>end,
            Or{end,..}=>end,
            Not{end,..}=>end,
            IsType{end,..}=>end,
            ObjectCreation{end,..}=>end,
            AnonFunction{end,..}=>end,
            Ref{end,..}=>end,
            RefMut{end,..}=>end,
            ForeverLoop{end,..}=>end,
            WhileLoop{end,..}=>end,
            ForLoop{end,..}=>end,
            Match{end,..}=>end,
        }
    }
}
#[derive(Debug)]
pub enum Data<'input> {
    String{start:Location,end:Location,inner:Cow<'input,str>},
    GenericNumber{start:Location,end:Location,negative:bool,inner:&'input str},
    GenericFloat{start:Location,end:Location,negative:bool,inner:&'input str},
    UInt{start:Location,end:Location,inner:u64},
    Int{start:Location,end:Location,inner:i64},
    Float{start:Location,end:Location,inner:f32},
    LargeFloat{start:Location,end:Location,inner:f64},
    Char{start:Location,end:Location,inner:char},
    Bool{start:Location,end:Location,inner:bool},
}
#[derive(Debug)]
pub enum MethodType {
    This,
    ThisMut,
    None,
}
#[derive(Debug)]
pub enum Import<'input> {
    Path{start:Location,end:Location,inner:Vec<&'input str>},
    PathBlock {
        start:Location,
        end:Location,
        path:Vec<&'input str>,
        block:Vec<Self>,
    },
}
#[derive(Debug)]
pub enum MatchPattern<'input> {
    Data{start:Location,end:Location,inner:Data<'input>},
    MethodCall {
        start:Location,
        end:Location,
        name:&'input str,
        args:Vec<Expr<'input>>,
    },
    Structure{start:Location,end:Location,inner:MatchPatternStructure<'input>},
    Var{start:Location,end:Location,inner:&'input str},
    Equal{start:Location,end:Location,inner:Expr<'input>},
    NotEqual{start:Location,end:Location,inner:Expr<'input>},
    GreaterEqual{start:Location,end:Location,inner:Expr<'input>},
    LessEqual{start:Location,end:Location,inner:Expr<'input>},
    Greater{start:Location,end:Location,inner:Expr<'input>},
    Less{start:Location,end:Location,inner:Expr<'input>},
    IsType{start:Location,end:Location,inner:Type<'input>},
}
#[derive(Debug)]
pub enum MatchPatternStructureItem<'input> {
    Field{start:Location,end:Location,inner:&'input str},
    NamedField {
        start:Location,
        end:Location,
        name:&'input str,
        rename:&'input str,
    },
    NamedBlock {
        start:Location,
        end:Location,
        name:&'input str,
        block:MatchPatternStructure<'input>,
    },
}
#[derive(Debug)]
pub enum MatchPatternStructure<'input> {
    Block {
        start:Location,
        end:Location,
        exact:bool,
        block:Vec<MatchPatternStructureItem<'input>>,
    },
    TypedBlock {
        start:Location,
        end:Location,
        exact:bool,
        type_name:&'input str,
        block:Vec<MatchPatternStructureItem<'input>>,
    },
}


#[derive(Debug)]
pub struct Block<'input> {
    pub start:Location,
    pub end:Location,
    pub inner:Vec<Statement<'input>>,
}
#[derive(Debug)]
pub struct Impl<'input> {
    pub start:Location,
    pub end:Location,
    pub params:Option<TypeParameters<'input>>,
    pub interface:Option<Type<'input>>,
    pub for_ty:Type<'input>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct Enum<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Option<TypeParameters<'input>>,
    pub variants:Vec<Type<'input>>,
}
#[derive(Debug)]
pub struct Match<'input> {
    pub start:Location,
    pub end:Location,
    pub to_match:Expr<'input>,
    pub leafs:Vec<(MatchPattern<'input>,Expr<'input>)>,
}
#[derive(Debug)]
pub struct AnonFunctionSignature<'input> {
    pub start:Location,
    pub end:Location,
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
}
impl<'input> AnonFunctionSignature<'input> {
    pub fn to_function(self,block:Block<'input>,start:Location,end:Location)->AnonFunction<'input> {
        let AnonFunctionSignature{params,ret_type,..}=self;
        AnonFunction {
            start,
            end,
            params,
            ret_type,
            block,
        }
    }
}
#[derive(Debug)]
pub struct AnonFunction<'input> {
    pub start:Location,
    pub end:Location,
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct FunctionSignature<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
}
impl<'input> FunctionSignature<'input> {
    pub fn to_function(self,block:Block<'input>,start:Location,end:Location)->Function<'input> {
        let FunctionSignature{public,name,params,ret_type,..}=self;
        Function {
            start,
            end,
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
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct Parameters<'input> {
    pub start:Location,
    pub end:Location,
    pub method_type:MethodType,
    pub normal:Vec<Parameter<'input>>,
    pub var_arg:Option<Parameter<'input>>,
}
#[derive(Debug)]
pub struct Parameter<'input> {
    pub start:Location,
    pub end:Location,
    pub mutable:bool,
    pub name:&'input str,
    pub ty:Type<'input>,
}
#[derive(Debug)]
pub struct TypeObjectField<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub ty:Type<'input>,
}
#[derive(Debug)]
pub struct Interface<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Option<TypeParameters<'input>>,
    pub requirement:Option<Type<'input>>,
    pub block:Block<'input>,
}
#[derive(Debug)]
pub struct TypeDef<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Option<TypeParameters<'input>>,
    pub ty:Type<'input>,
}
#[derive(Debug)]
pub struct TypeParameters<'input> {
    pub start:Location,
    pub end:Location,
    pub params:Vec<TypeParameter<'input>>,
}
#[derive(Debug)]
pub struct TypeParameter<'input> {
    pub start:Location,
    pub end:Location,
    pub name:&'input str,
    pub ty:Option<Type<'input>>,
}
#[derive(Debug)]
pub struct VarDef<'input> {
    pub start:Location,
    pub end:Location,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub ty:Option<Type<'input>>,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct StaticVarDef<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub ty:Type<'input>,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct ConstVarDef<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub ty:Type<'input>,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct VarAssign<'input> {
    pub start:Location,
    pub end:Location,
    pub name:&'input str,
    pub data:Expr<'input>,
}
#[derive(Debug)]
pub struct ObjectField<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub mutable:Option<Visibility>,
    pub name:&'input str,
    pub data:Expr<'input>,
}
