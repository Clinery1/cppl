use std::{
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
};
use cppl_error::Location;


#[derive(Debug)]
pub enum VarScopeItem<'input> {
    Root {
        modules:Vec<&'input str>,
        imports:Vec<&'input str>,
        statements:Vec<Statement<'input>>,
        vars:HashMap<&'input str,Vec<Scope>>,
    },
    AnonFunction {
        imports:Vec<&'input str>,
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        params:Vec<Scope>,
        ret_type:Option<Type<'input>>,
        statements:Vec<Statement<'input>>,
        vars:HashMap<&'input str,Vec<Scope>>,
    },
    Function {
        imports:Vec<&'input str>,
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        name:&'input str,
        public:Option<Visibility>,
        params:Vec<Scope>,
        ret_type:Option<Type<'input>>,
        statements:Vec<Statement<'input>>,
        vars:HashMap<&'input str,Vec<Scope>>,
    },
    MatchBlockVar {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        var:&'input str,
        statements:Vec<Statement<'input>>,
    },
    Block {
        imports:Vec<&'input str>,
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        statements:Vec<Statement<'input>>,
        vars:HashMap<&'input str,Vec<Scope>>,
    },
    Interface {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        name:&'input str,
        /// Index into self.statements
        required_functions:Vec<usize>,
        /// Index into self.statements
        optional_functions:Vec<usize>,
        public:Option<Visibility>,
        params:Option<TypeParameters<'input>>,
        requirement:Option<Type<'input>>,
        statements:Vec<Statement<'input>>,
    },
    Impl {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        interface:Option<Type<'input>>,
        params:Option<TypeParameters<'input>>,
        for_ty:Type<'input>,
        statements:Vec<Statement<'input>>,
    },
    Parameter {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        mutable:bool,
        name:&'input str,
        ty:Type<'input>,
    },
    Var {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        mutable:Option<Visibility>,
        name:&'input str,
        ty:Type<'input>,
        data:Expr<'input>,
    },
    Const {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        public:Option<Visibility>,
        name:&'input str,
        ty:Type<'input>,
        data:Expr<'input>,
    },
    Static {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        public:Option<Visibility>,
        mutable:Option<Visibility>,
        name:&'input str,
        ty:Type<'input>,
        data:Expr<'input>,
    },
    Type {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        public:Option<Visibility>,
        name:&'input str,
        ty:Type<'input>,
        params:Option<TypeParameters<'input>>,
    },
    Enum {
        parent_scope:Scope,
        def_start:Location,
        def_end:Location,
        public:Option<Visibility>,
        params:Option<TypeParameters<'input>>,
        variants:Vec<Type<'input>>,
    },
}
impl<'input> VarScopeItem<'input> {
    pub fn add_var(&mut self,var:&'input str,scope:Scope) {
        use VarScopeItem::*;
        match self {
            Root{vars,..}|Function{vars,..}|AnonFunction{vars,..}|Block{vars,..}=>{
                vars.entry(var).or_insert(Vec::new()).push(scope);
            },
            _=>{},
        }
    }
    pub fn add_module(&mut self,module:&'input str) {
        use VarScopeItem::*;
        match self {
            Root{modules,..}=>modules.push(module),
            _=>{},
        }
    }
    pub fn add_import(&mut self,import:&'input str) {
        use VarScopeItem::*;
        match self {
            Root{imports,..}|Function{imports,..}|AnonFunction{imports,..}|Block{imports,..}=>imports.push(import),
            _=>{},
        }
    }
    pub fn get_var(&mut self,var:&'input str)->Option<Scope> {
        use VarScopeItem::*;
        match self {
            Root{vars,..}|Function{vars,..}|AnonFunction{vars,..}|Block{vars,..}=>vars.get(var).map(|v|v.last()).flatten().map(|s|*s),
            _=>None,
        }
    }
    pub fn add_stmt(&mut self,stmt:Statement<'input>) {
        use VarScopeItem::*;
        match self {
            Interface{statements,optional_functions,required_functions,..}=>{
                match &stmt {
                    Statement::FunctionDef{..}=>optional_functions.push(statements.len()),
                    Statement::FunctionSig{..}=>required_functions.push(statements.len()),
                    _=>{},
                }
                statements.push(stmt);
            },
            Impl{statements,..}|
                Root{statements,..}|
                Function{statements,..}|
                AnonFunction{statements,..}|
                Block{statements,..}=>statements.push(stmt),
            _=>{},
        }
    }
}


#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Visibility {
    Library,
    Local,
    Full,
}
#[derive(Debug)]
pub enum Type<'input> {
    UnknownNamed {
        name:&'input str,
        start:Location,
        end:Location,
        generics:Vec<Self>,
    },
    Named {
        path:Scope,
        start:Location,
        end:Location,
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
        items:Vec<Self>,
    },
    Composite {
        start:Location,
        end:Location,
        items:Vec<Self>,
    },
    FunctionSig {
        start:Location,
        end:Location,
        inner:Box<AnonFunctionSignature<'input>>,
    },
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
    FunctionDef {
        start:Location,
        end:Location,
        def:Scope,
    },
    FunctionSig {
        start:Location,
        end:Location,
        sig:FunctionSignature<'input>,
    },
    InterfaceDef {
        start:Location,
        end:Location,
        def:Scope,
    },
    TypeDef {
        start:Location,
        end:Location,
        def:Scope,
    },
    VarDef {
        start:Location,
        end:Location,
        def:Scope,
    },
    StaticVarDef {
        start:Location,
        end:Location,
        def:Scope,
    },
    ConstVarDef {
        start:Location,
        end:Location,
        def:Scope,
    },
    VarAssign {
        start:Location,
        end:Location,
        loc:Scope,
        data:Expr<'input>,
    },
    UnknownVarAssign {
        start:Location,
        end:Location,
        name:&'input str,
        data:Expr<'input>,
    },
    Expr {
        start:Location,
        end:Location,
        expr:Expr<'input>,
    },
    Return {
        start:Location,
        end:Location,
        label:Option<&'input str>,
        val:Option<Expr<'input>>,
    },
    Continue {
        start:Location,
        end:Location,
        label:Option<&'input str>,
    },
    Enum {
        start:Location,
        end:Location,
        e:Scope,
    },
    Impl {
        start:Location,
        end:Location,
        i:Scope,
    },
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
    UnknownFunctionCall{
        path:Vec<&'input str>,
        start:Location,
        end:Location,
        args:Vec<Self>,
    },
    FunctionCall{
        path:Scope,
        start:Location,
        end:Location,
        args:Vec<Self>,
    },
    UnknownAssociatedPath {
        start:Location,
        end:Location,
        path:Vec<&'input str>,
    },
    AssociatedPath {
        start:Location,
        end:Location,
        path:Scope,
    },
    Var {
        start:Location,
        end:Location,
        path:Scope,
    },
    UnknownVar {
        start:Location,
        end:Location,
        name:&'input str,
    },
    Block {
        start:Location,
        end:Location,
        block:Scope,
    },
    Data {
        start:Location,
        end:Location,
        data:Data<'input>,
    },
    Add {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Sub {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Mul {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Div {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Mod {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Negate {
        start:Location,
        end:Location,
        inner:Box<Self>,
    },
    Equal {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    NotEqual {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    GreaterEqual {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    LessEqual {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Greater {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Less {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    And {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Or {
        start:Location,
        end:Location,
        inner:Box<[Self;2]>,
    },
    Not {
        start:Location,
        end:Location,
        inner:Box<Self>,
    },
    IsType {
        start:Location,
        end:Location,
        to_test:Box<Self>,
        ty:Type<'input>,
    },
    ObjectCreation {
        start:Location,
        end:Location,
        fields:Vec<ObjectField<'input>>,
    },
    AnonFunction {
        start:Location,
        end:Location,
        function:Scope,
    },
    Ref {
        start:Location,
        end:Location,
        val:Box<Self>,
    },
    RefMut {
        start:Location,
        end:Location,
        val:Box<Self>,
    },
    ForeverLoop {
        start:Location,
        end:Location,
        block:Scope,
    },
    WhileLoop {
        start:Location,
        end:Location,
        condition:Box<Self>,
        block:Scope,
    },
    ForLoop {
        start:Location,
        end:Location,
        var:&'input str,
        iterator:Box<Self>,
        block:Scope,
    },
    Match {
        start:Location,
        end:Location,
        block:Box<Match<'input>>,
    },
}
#[derive(Debug)]
pub enum Data<'input> {
    String {
        start:Location,
        end:Location,
        s:Cow<'input,str>,
    },
    GenericNumber {
        start:Location,
        end:Location,
        negative:bool,
        data:&'input str,
    },
    GenericFloat {
        start:Location,
        end:Location,
        negative:bool,
        data:&'input str,
    },
    UInt {
        start:Location,
        end:Location,
        data:u64,
    },
    Int {
        start:Location,
        end:Location,
        data:i64,
    },
    Float {
        start:Location,
        end:Location,
        data:f32,
    },
    LargeFloat {
        start:Location,
        end:Location,
        data:f64,
    },
    Char {
        start:Location,
        end:Location,
        data:char,
    },
    Bool {
        start:Location,
        end:Location,
        data:bool,
    },
}
#[derive(Debug)]
pub enum MethodType {
    This,
    ThisMut,
    None,
}
#[derive(Debug)]
pub enum MatchPattern<'input> {
    Data {
        start:Location,
        end:Location,
        inner:Data<'input>,
    },
    MethodCall {
        start:Location,
        end:Location,
        name:&'input str,
        args:Vec<Expr<'input>>,
    },
    Structure {
        start:Location,
        end:Location,
        structure:MatchPatternStructure<'input>,
    },
    Var {
        start:Location,
        end:Location,
        name:&'input str,
    },
    Equal {
        start:Location,
        end:Location,
        inner:Expr<'input>,
    },
    NotEqual {
        start:Location,
        end:Location,
        inner:Expr<'input>,
    },
    GreaterEqual {
        start:Location,
        end:Location,
        inner:Expr<'input>,
    },
    LessEqual {
        start:Location,
        end:Location,
        inner:Expr<'input>,
    },
    Greater {
        start:Location,
        end:Location,
        inner:Expr<'input>,
    },
    Less {
        start:Location,
        end:Location,
        inner:Expr<'input>,
    },
    IsType {
        start:Location,
        end:Location,
        inner:Type<'input>,
    },
}
#[derive(Debug)]
pub enum MatchPatternStructureItem<'input> {
    Field {
        start:Location,
        end:Location,
        name:&'input str,
    },
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


#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
pub struct TypeIdent;
#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
pub struct VarIdent;
#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
pub struct Scope(pub u64);
#[derive(Debug,Default)]
pub struct Scopes<'input> {
    pub var_scope_count:u64,
    pub var_scopes:HashMap<Scope,VarScopeItem<'input>>,
}
impl<'input> Scopes<'input> {
    pub fn get_mut(&mut self,scope:Scope)->Option<&mut VarScopeItem<'input>> {
        self.var_scopes.get_mut(&scope)
    }
    pub fn push(&mut self,item:VarScopeItem<'input>)->Scope {
        let scope=Scope(self.var_scope_count);
        self.var_scope_count+=1;
        self.var_scopes.insert(scope,item);
        return scope;
    }
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
#[derive(Debug)]
pub struct FunctionSignature<'input> {
    pub start:Location,
    pub end:Location,
    pub public:Option<Visibility>,
    pub name:&'input str,
    pub params:Parameters<'input>,
    pub ret_type:Option<Type<'input>>,
}
#[derive(Debug)]
pub struct Parameters<'input> {
    pub start:Location,
    pub end:Location,
    pub method_type:MethodType,
    pub normal:Vec<Parameter<'input>>,
    pub var_arg:Option<Parameter<'input>>,
}
impl<'input> Parameters<'input> {
    pub fn names(&self)->Vec<&'input str> {
        let mut names=Vec::new();
        for p in self.normal.iter() {
            names.push(p.name);
        }
        if let Some(va)=&self.var_arg {
            names.push(va.name);
        }
        return names;
    }
}
#[derive(Debug)]
pub struct Parameter<'input> {
    pub name:&'input str,
    pub start:Location,
    pub end:Location,
    pub mutable:bool,
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
pub struct TypeParameters<'input> {
    pub start:Location,
    pub end:Location,
    pub params:Vec<TypeParameter<'input>>,
}
#[derive(Debug)]
pub struct TypeParameter<'input> {
    pub name:&'input str,
    pub start:Location,
    pub end:Location,
    pub ty:Option<Type<'input>>,
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
