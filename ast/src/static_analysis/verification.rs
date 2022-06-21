use cppl_error::{
    Location,
    Error,
};
use crate::{
    raw::*,
    is_camel_case,
    is_snake_case,
    is_screaming_snake_case,
};


trait VerifyAst<'input> {
    fn verify(&mut self,state:&mut State<'input>);
}
impl<'input> VerifyAst<'input> for Statement<'input> {
    fn verify(&mut self,state:&mut State<'input>) {
        use Statement as St;
        match self {
            St::TypeDef{inner:TypeDef{start,end,name,params,..},..}=>{
                if !is_camel_case(name) {
                    state.push_warn(*start,*end,"types should have a CamelCase name");
                }
                if let Some(TypeParameters{params,..})=params {
                    params.iter_mut().for_each(|TypeParameter{start,end,name,..}|{
                        if !is_camel_case(name) {
                            state.push_warn(*start,*end,"type parameters should have a CamelCase name");
                        }
                    });
                }
            },
            St::FunctionSig{inner:FunctionSignature{start,end,name,params,..},..}=>{
                if let Some(Parameter{start,end,name,..})=params.var_arg {
                    if !is_snake_case(name) {
                        state.push_warn(start,end,"parameters should have a snake_case name");
                    }
                }
                params.normal.iter_mut().for_each(|Parameter{start,end,name,..}|{
                    if !is_snake_case(name) {
                        state.push_warn(*start,*end,"parameters should have a snake_case name");
                    }
                });
                if !is_snake_case(name) {
                    state.push_warn(*start,*end,"functions should have a snake_case name");
                }
            },
            St::FunctionDef{inner:Function{start,end,name,params,block:Block{inner,..},..},..}=>{
                if let Some(Parameter{start,end,name,..})=params.var_arg {
                    if !is_snake_case(name) {
                        state.push_warn(start,end,"parameters should have a snake_case name");
                    }
                }
                params.normal.iter_mut().for_each(|Parameter{start,end,name,..}|{
                    if !is_snake_case(name) {
                        state.push_warn(*start,*end,"parameters should have a snake_case name");
                    }
                });
                if !is_snake_case(name) {
                    state.push_warn(*start,*end,"functions should have a snake_case name");
                }
                state.scoped();
                inner.iter_mut().for_each(|s|s.verify(state));
                state.drop_scope();
            },
            St::VarDef{start,end,inner:VarDef{mutable,name,..},..}=>if state.is_root() {
                state.push(*start,*end,"variable definitions are not allowed in the root scope");
            } else {
                if !is_snake_case(name) {
                    state.push_warn(*start,*end,"variables should have a snake_case name");
                }
                match mutable {
                    Some(Visibility::Full{..})|None=>{},
                    _=>{
                        state.push_warn(*start,*end,"regular variables are not allowed to have \"lib\" or \"local\" mutability. Setting mutability to None.");
                        *mutable=None;
                    },
                }
            },
            St::StaticVarDef{start,end,inner:StaticVarDef{name,data,..},..}=>{
                if !is_screaming_snake_case(name) {
                    state.push_warn(*start,*end,"static variables should have a SCREAMING_SNAKE_CASE name");
                }
                data.verify(state);
            },
            St::ConstVarDef{start,end,inner:ConstVarDef{name,data,..},..}=>{
                if !is_screaming_snake_case(name) {
                    state.push_warn(*start,*end,"const variables should have a SCREAMING_SNAKE_CASE name");
                }
                data.verify(state);
            },
            St::VarAssign{start,end,..}=>if state.is_root() {
                state.push(*start,*end,"assigning to variables is not allowed in the root scope");
            },
            St::Return{start,end,..}=>if state.is_root() {
                state.push(*start,*end,"return statements are not allowed in the root scope");
            },
            St::Continue{start,end,..}=>if state.is_root() {
                state.push(*start,*end,"continue statements are not allowed in the root scope");
            },
            St::InterfaceDef{inner:Interface{start,end,name,params,block:Block{inner,..},..},..}=>{
                if let Some(TypeParameters{params,..})=params {
                    params.iter_mut().for_each(|TypeParameter{start,end,name,..}|{
                        if !is_camel_case(name) {
                            state.push_warn(*start,*end,"type parameters should have a CamelCase name");
                        }
                    });
                }
                if !is_camel_case(name) {
                    state.push_warn(*start,*end,"interfaces should have a CamelCase name");
                }
                state.scoped();
                for i in inner {
                    match i {
                        St::FunctionDef{..}=>i.verify(state),
                        St::FunctionSig{..}|St::TypeDef{..}=>{},
                        s=>state.push(s.start(),s.end(),"invalid statement in interface definition"),
                    }
                }
                state.drop_scope();
            },
            St::Enum{inner:Enum{start,end,name,variants,params,..},..}=>{
                if let Some(TypeParameters{params,..})=params {
                    params.iter_mut().for_each(|TypeParameter{start,end,name,..}|{
                        if !is_camel_case(name) {
                            state.push_warn(*start,*end,"type parameters should have a CamelCase name");
                        }
                    });
                }
                if !is_camel_case(name) {
                    state.push_warn(*start,*end,"enums should have a CamelCase name");
                }
                for t in variants {
                    use Type::*;
                    match t {
                        Named{..}|Uint{..}|Int{..}|Float{..}|String{..}|DoubleFloat{..}|Char{..}|Bool{..}|Byte{..}|Never{..}=>{},
                        t=>state.push(t.start(),t.end(),"invalid type in enum definition"),
                    }
                }
            },
            St::Impl{inner:Impl{block:Block{inner,..},..},..}=>{
                state.scoped();
                for i in inner {
                    match i {
                        St::FunctionDef{..}=>i.verify(state),
                        St::TypeDef{..}=>{},
                        s=>state.push(s.start(),s.end(),"invalid statement in interface definition"),
                    }
                }
                state.drop_scope();
            },
            St::Expr{start,end,inner}=>if state.is_root() {
                state.push(*start,*end,"expr statements are not allowed in the root scope");
            } else {
                inner.verify(state);
            },
            St::Module{start,end,..}=>if !state.is_root() {
                state.push(*start,*end,"mod statements are **ONLY** allowed in the root scope");
            },
            _=>{},
        }
    }
}
impl<'input> VerifyAst<'input> for Expr<'input> {
    fn verify(&mut self,state:&mut State<'input>) {
        use Expr as E;
        match self {
            E::FieldAccess{from,..}=>from.verify(state),
            E::MethodCall{from,args,..}=>{
                from.verify(state);
                args.iter_mut().for_each(|e|e.verify(state));
            },
            E::FunctionCall{args,..}=>args.iter_mut().for_each(|e|e.verify(state)),
            E::Block{inner,..}=>inner.verify(state),
            E::Add{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Sub{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Mul{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Div{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Mod{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Negate{inner,..}=>inner.verify(state),
            E::Equal{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::NotEqual{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::GreaterEqual{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::LessEqual{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Greater{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Less{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::And{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Or{inner,..}=>inner.iter_mut().for_each(|e|e.verify(state)),
            E::Not{inner,..}=>inner.verify(state),
            E::IsType{inner,..}=>inner.verify(state),
            E::ObjectCreation{inner,..}=>{
                inner.iter_mut().for_each(|f|f.data.verify(state));
            },
            E::AnonFunction{inner,..}=>{
                inner.verify(state);
            },
            E::Ref{inner,..}=>inner.verify(state),
            E::RefMut{inner,..}=>inner.verify(state),
            E::ForeverLoop{inner,..}=>inner.verify(state),
            E::WhileLoop{condition,block,..}=>{
                condition.verify(state);
                block.verify(state);
            },
            E::ForLoop{iterator,block,..}=>{
                iterator.verify(state);
                block.verify(state);
            },
            E::Match{inner,..}=>inner.verify(state),
            _=>{},
        }
    }
}
impl<'input> VerifyAst<'input> for Block<'input> {
    fn verify(&mut self,state:&mut State<'input>) {
        self.inner.iter_mut().for_each(|s|s.verify(state.scoped()));
        state.drop_scope();
    }
}
impl<'input> VerifyAst<'input> for Match<'input> {
    fn verify(&mut self,state:&mut State<'input>) {
        self.to_match.verify(state);
        self.leafs.iter_mut().for_each(|(pat,expr)|{
            pat.verify(state);
            expr.verify(state);
        });
    }
}
impl<'input> VerifyAst<'input> for MatchPattern<'input> {
    fn verify(&mut self,state:&mut State<'input>) {
        use MatchPattern as MP;
        match self {
            MP::MethodCall{args,..}=>args.iter_mut().for_each(|e|e.verify(state)),
            MP::Equal{inner,..}=>inner.verify(state),
            MP::NotEqual{inner,..}=>inner.verify(state),
            MP::GreaterEqual{inner,..}=>inner.verify(state),
            MP::LessEqual{inner,..}=>inner.verify(state),
            MP::Greater{inner,..}=>inner.verify(state),
            MP::Less{inner,..}=>inner.verify(state),
            _=>{},
        }
    }
}
impl<'input> VerifyAst<'input> for AnonFunction<'input> {
    fn verify(&mut self,state:&mut State<'input>) {
        if let Some(Parameter{start,end,name,..})=self.params.var_arg {
            if !is_snake_case(name) {
                state.push_warn(start,end,"parameters should have a snake_case name");
            }
        }
        self.params.normal.iter_mut().for_each(|Parameter{start,end,name,..}|{
            if !is_snake_case(name) {
                state.push_warn(*start,*end,"parameters should have a snake_case name");
            }
        });
        self.block.verify(state);
    }
}


#[derive(PartialEq,Copy,Clone)]
enum Scope {
    Root,
    Level(usize),
}
impl Scope {
    pub fn next(self)->Self {
        use Scope::*;
        match self {
            Level(num)=>Level(num+1),
            _=>Level(0),
        }
    }
    pub fn prev(self)->Self {
        use Scope::*;
        match self {
            Level(0)=>Root,
            Level(num)=>Level(num-1),
            _=>Root,
        }
    }
    pub fn is_root(&self)->bool {
        match self {
            Scope::Root=>true,
            _=>false,
        }
    }
}


struct State<'input> {
    pub scope:Scope,
    pub filename:&'input str,
    pub errors:Vec<Error<'input,String>>,
    pub warnings:Vec<Error<'input,String>>,
}
impl<'input> State<'input> {
    pub fn new(filename:&'input str)->Self {
        State {
            scope:Scope::Root,
            filename,
            errors:Vec::new(),
            warnings:Vec::new(),
        }
    }
    pub fn push<T:Into<String>>(&mut self,start:Location,end:Location,err:T) {
        self.errors.push(Error::new_verif(self.filename,start,end,err.into()));
    }
    pub fn push_warn<T:Into<String>>(&mut self,start:Location,end:Location,err:T) {
        self.warnings.push(Error::new_warning(self.filename,start,end,err.into()));
    }
    pub fn scoped(&mut self)->&mut Self {
        self.scope=self.scope.next();
        return self;
    }
    #[inline]
    pub fn drop_scope(&mut self) {
        self.scope=self.scope.prev();
    }
    #[inline]
    pub fn is_root(&self)->bool {
        self.scope.is_root()
    }
}


pub fn verify<'input>(filename:&'input str,statements:&mut [Statement<'input>])->Result<Vec<Error<'input,String>>,[Vec<Error<'input,String>>;2]> {
    let mut state=State::new(filename);
    statements.into_iter().for_each(|s|s.verify(&mut state));
    if state.errors.len()==0 {
        return Ok(state.warnings);
    } else {
        return Err([state.errors,state.warnings]);
    }
}
