#![allow(unused_variables)]
use std::collections::HashMap;
use cppl_error::Error;
use crate::{
    refined::*,
    raw,
};


trait Convert<'input> {
    type Output;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>>;
}
impl<'input,T:Convert<'input>> Convert<'input> for Option<T> {
    type Output=Option<T::Output>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        Ok(match self {
            Some(c)=>Some(c.convert(scopes,parent,filename)?),
            None=>None,
        })
    }
}
impl<'input> Convert<'input> for raw::MethodType {
    type Output=MethodType;
    fn convert(self,_:&mut Scopes<'input>,_:Scope,_:&'input str)->Result<Self::Output,Error<'input,String>> {
        Ok(match self {
            raw::MethodType::This{..}=>MethodType::This,
            raw::MethodType::ThisMut{..}=>MethodType::ThisMut,
            raw::MethodType::None{..}=>MethodType::None,
        })
    }
}
impl<'input> Convert<'input> for raw::Visibility {
    type Output=Visibility;
    fn convert(self,_:&mut Scopes<'input>,_:Scope,_:&'input str)->Result<Self::Output,Error<'input,String>> {
        Ok(match self {
            raw::Visibility::Full{..}=>Visibility::Full,
            raw::Visibility::Library{..}=>Visibility::Library,
            raw::Visibility::Local{..}=>Visibility::Local,
        })
    }
}
impl<'input> Convert<'input> for raw::Parameter<'input> {
    type Output=Parameter<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let raw::Parameter{start,end,mutable,name,ty}=self;
        let ty=ty.convert(scopes,parent,filename)?;
        return Ok(Parameter{start,end,name,ty,mutable});
    }
}
impl<'input> Convert<'input> for raw::Parameters<'input> {
    type Output=Parameters<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let raw::Parameters{start,end,method_type,normal,var_arg}=self;
        let mut new_normal=Vec::new();
        for p in normal.into_iter() {
            new_normal.push(p.convert(scopes,parent,filename)?);
        }
        let var_arg=var_arg.convert(scopes,parent,filename)?;
        let method_type=method_type.convert(scopes,parent,filename)?;
        return Ok(Parameters{start,end,method_type,normal:new_normal,var_arg});
    }
}
impl<'input> Convert<'input> for raw::TypeParameter<'input> {
    type Output=TypeParameter<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let raw::TypeParameter{start,end,name,ty}=self;
        let ty=ty.convert(scopes,parent,filename)?;
        return Ok(TypeParameter{start,end,name,ty});
    }
}
impl<'input> Convert<'input> for raw::TypeParameters<'input> {
    type Output=TypeParameters<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let raw::TypeParameters{start,end,params}=self;
        let mut new_params=Vec::new();
        for p in params.into_iter() {
            new_params.push(p.convert(scopes,parent,filename)?);
        }
        return Ok(TypeParameters{start,end,params:new_params});
    }
}
impl<'input> Convert<'input> for raw::TypeObjectField<'input> {
    type Output=TypeObjectField<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let raw::TypeObjectField{start,end,public,mutable,name,ty}=self;
        let public=public.convert(scopes,parent,filename)?;
        let mutable=mutable.convert(scopes,parent,filename)?;
        let ty=ty.convert(scopes,parent,filename)?;
        return Ok(TypeObjectField{start,end,public,mutable,name,ty});
    }
}
impl<'input> Convert<'input> for raw::AnonFunctionSignature<'input> {
    type Output=AnonFunctionSignature<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let raw::AnonFunctionSignature{start,end,params,ret_type}=self;
        let params=params.convert(scopes,parent,filename)?;
        let ret_type=ret_type.convert(scopes,parent,filename)?;
        return Ok(AnonFunctionSignature{start,end,params,ret_type});
    }
}
impl<'input> Convert<'input> for raw::Type<'input> {
    type Output=Type<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        Ok(match self {
            raw::Type::Named{start,end,name,generics:old_generics}=>{
                let mut generics=Vec::new();
                for t in old_generics {
                    generics.push(t.convert(scopes,parent,filename)?);
                }
                Type::UnknownNamed{start,end,name,generics}
            },
            raw::Type::Object{start,end,fields:old_fields,exact}=>{
                let mut fields=Vec::new();
                for f in old_fields {
                    fields.push(f.convert(scopes,parent,filename)?);
                }
                Type::Object{start,end,fields,exact}
            },
            raw::Type::Union{start,end,inner}=>{
                let mut items=Vec::new();
                for i in inner {
                    items.push(i.convert(scopes,parent,filename)?);
                }
                Type::Union{start,end,items}
            },
            raw::Type::Composite{start,end,inner}=>{
                let mut items=Vec::new();
                for i in inner {
                    items.push(i.convert(scopes,parent,filename)?);
                }
                Type::Composite{start,end,items}
            },
            raw::Type::FunctionSig{start,end,inner}=>Type::FunctionSig{start,end,inner:Box::new(inner.convert(scopes,parent,filename)?)},
            raw::Type::Uint{..}=>Type::Uint,
            raw::Type::Int{..}=>Type::Int,
            raw::Type::Float{..}=>Type::Float,
            raw::Type::DoubleFloat{..}=>Type::DoubleFloat,
            raw::Type::Byte{..}=>Type::Byte,
            raw::Type::Bool{..}=>Type::Bool,
            raw::Type::Char{..}=>Type::Char,
            raw::Type::String{..}=>Type::String,
            raw::Type::Never{..}=>Type::Never,
            _=>unreachable!(),
        })
    }
}
impl<'input> Convert<'input> for raw::Statement<'input> {
    type Output=();
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        use raw::Statement as S;
        match self {
            S::FunctionDef{inner:raw::Function{start,end,public,name,params,ret_type,block},..}=>{
                let ret_type=ret_type.convert(scopes,parent,filename)?;
                let public=public.convert(scopes,parent,filename)?;
                let params=params.convert(scopes,parent,filename)?;
                let scope=scopes.push(VarScopeItem::Function {
                    imports:Vec::new(),
                    parent_scope:parent,
                    def_start:start,
                    def_end:end,
                    public,
                    params:Vec::new(),
                    name,
                    ret_type,
                    statements:Vec::new(),
                    vars:HashMap::new(),
                });
                for param in params.normal.into_iter() {
                    let name=param.name;
                    if scopes.get_mut(scope).unwrap().get_var(param.name).is_some() {
                        return Err(Error::new_verif(filename,start,end,format!("parameter {} is specified twice",name)));
                    }
                    let param_scope=scopes.push(VarScopeItem::Parameter {
                        parent_scope:parent,
                        def_start:param.start,
                        def_end:param.end,
                        mutable:param.mutable,
                        name,
                        ty:param.ty,
                    });
                    scopes.get_mut(scope).unwrap().add_var(name,param_scope);
                }
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::FunctionDef{start,end,def:scope});
                for s in block.inner {
                    s.convert(scopes,scope,filename)?;
                }
            },
            S::FunctionSig{inner:raw::FunctionSignature{start,end,public,name,params,ret_type},..}=>{
                let ret_type=ret_type.convert(scopes,parent,filename)?;
                let public=public.convert(scopes,parent,filename)?;
                let params=params.convert(scopes,parent,filename)?;
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::FunctionSig{
                    start,
                    end,
                    sig:FunctionSignature {
                        start,
                        end,
                        public,
                        name,
                        params,
                        ret_type,
                    },
                });
            },
            S::InterfaceDef{inner:raw::Interface{start,end,public,name,params,requirement,block},..}=>{
                let public=public.convert(scopes,parent,filename)?;
                let params=params.convert(scopes,parent,filename)?;
                let requirement=requirement.convert(scopes,parent,filename)?;
                let scope=scopes.push(VarScopeItem::Interface {
                    parent_scope:parent,
                    def_start:start,
                    def_end:end,
                    public,
                    name,
                    params,
                    requirement,
                    statements:Vec::new(),
                    optional_functions:Vec::new(),
                    required_functions:Vec::new(),
                });
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::InterfaceDef{start,end,def:scope});
                for i in block.inner.into_iter().map(|s|s.convert(scopes,scope,filename)) {
                    i?;
                }
            },
            S::TypeDef{inner:raw::TypeDef{start,end,public,name,params,ty},..}=>{
                let public=public.convert(scopes,parent,filename)?;
                let ty=ty.convert(scopes,parent,filename)?;
                let params=params.convert(scopes,parent,filename)?;
                let scope=scopes.push(VarScopeItem::Type {
                    parent_scope:parent,
                    def_start:start,
                    def_end:end,
                    public,
                    name,
                    params,
                    ty,
                });
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::TypeDef{start,end,def:scope});
            },
            S::VarDef{inner:raw::VarDef{start,end,mutable,name,ty,data},..}=>{
                let mutable=mutable.convert(scopes,parent,filename)?;
                let ty=ty.convert(scopes,parent,filename)?.unwrap_or(Type::Unknown);
                let data=data.convert(scopes,parent,filename)?;
                let scope=scopes.push(VarScopeItem::Var {
                    parent_scope:parent,
                    def_start:start,
                    def_end:end,
                    mutable,
                    name,
                    ty,
                    data,
                });
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::VarDef{start,end,def:scope});
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_var(name,scope);
            },
            S::StaticVarDef{inner:raw::StaticVarDef{start,end,public,mutable,name,ty,data},..}=>{
                let public=public.convert(scopes,parent,filename)?;
                let mutable=mutable.convert(scopes,parent,filename)?;
                let ty=ty.convert(scopes,parent,filename)?;
                let data=data.convert(scopes,parent,filename)?;
                let scope=scopes.push(VarScopeItem::Static {
                    parent_scope:parent,
                    def_start:start,
                    def_end:end,
                    public,
                    mutable,
                    name,
                    ty,
                    data,
                });
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::StaticVarDef{start,end,def:scope});
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_var(name,scope);
            },
            S::ConstVarDef{inner:raw::ConstVarDef{start,end,public,name,ty,data},..}=>{
                let public=public.convert(scopes,parent,filename)?;
                let ty=ty.convert(scopes,parent,filename)?;
                let data=data.convert(scopes,parent,filename)?;
                let scope=scopes.push(VarScopeItem::Const {
                    parent_scope:parent,
                    def_start:start,
                    def_end:end,
                    public,
                    name,
                    ty,
                    data,
                });
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::ConstVarDef{start,end,def:scope});
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_var(name,scope);
            },
            S::VarAssign{inner:raw::VarAssign{start,end,name,data},..}=>{
                let data=data.convert(scopes,parent,filename)?;
                if let Some(loc)=scopes.get_mut(parent).expect("Internal compiler error: invalid scope").get_var(name) {
                    scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::VarAssign {
                        start,
                        end,
                        data,
                        loc,
                    });
                } else {
                    return Err(Error::new_verif(filename,start,end,format!("variable {} is not defined",name)));
                }
            },
            S::Expr{start,end,inner}=>{
                let expr=inner.convert(scopes,parent,filename)?;
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::Expr {
                    start,
                    end,
                    expr,
                });
            },
            S::Import{start:_,end:_,inner:_}=>{
                todo!();
            },
            S::Return{start,end,label,val}=>{
                let val=val.convert(scopes,parent,filename)?;
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::Return {
                    start,
                    end,
                    label,
                    val,
                });
            },
            S::Continue{start,end,inner}=>scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::Continue{start,end,label:inner}),
            S::Enum{start:_,end:_,inner:_}=>{
                todo!();
            },
            S::Module{inner,..}=>scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_module(inner),
            S::Impl{start:_,end:_,inner:_}=>{
                todo!();
            },
        }
        return Ok(());
    }
}
impl<'input> Convert<'input> for raw::Expr<'input> {
    type Output=Expr<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        Ok(match self {
            raw::Expr::FieldAccess{start,end,from,name}=>{
                let from=Box::new(from.convert(scopes,parent,filename)?);
                Expr::FieldAccess{start,end,from,name}
            },
            raw::Expr::MethodCall{start,end,from,name,args:old_args}=>{
                let from=Box::new(from.convert(scopes,parent,filename)?);
                let mut args=Vec::with_capacity(old_args.len());
                for arg in old_args {
                    args.push(arg.convert(scopes,parent,filename)?);
                }
                Expr::MethodCall{start,end,from,name,args}
            },
            raw::Expr::FunctionCall{start,end,path,args:old_args}=>{
                let mut args=Vec::with_capacity(old_args.len());
                for arg in old_args {
                    args.push(arg.convert(scopes,parent,filename)?);
                }
                Expr::UnknownFunctionCall{start,end,path,args}
            },
            raw::Expr::AssociatedPath{start,end,inner}=>{
                Expr::UnknownAssociatedPath{start,end,path:inner}
            },
            raw::Expr::Var{start,end,inner}=>{
                if let Some(path)=scopes.get_mut(parent).expect("Internal compiler error: invalid scope").get_var(inner) {
                    Expr::Var{start,end,path}
                } else {
                    return Err(Error::new_verif(filename,start,end,format!("variable {} is not defined",inner)));
                }
            },
            raw::Expr::Block{start,end,inner}=>{
                let block=inner.convert(scopes,parent,filename)?;
                Expr::Block{start,end,block}
            },
            raw::Expr::Data{start,end,inner}=>{
                let data=inner.convert(scopes,parent,filename)?;
                Expr::Data{start,end,data}
            },
            raw::Expr::Add{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Add{start,end,inner}
            },
            raw::Expr::Sub{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Sub{start,end,inner}
            },
            raw::Expr::Mul{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Mul{start,end,inner}
            },
            raw::Expr::Div{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Div{start,end,inner}
            },
            raw::Expr::Mod{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Mod{start,end,inner}
            },
            raw::Expr::Negate{start,end,inner}=>{
                let inner=Box::new(inner.convert(scopes,parent,filename)?);
                Expr::Negate{start,end,inner}
            },
            raw::Expr::Equal{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Equal{start,end,inner}
            },
            raw::Expr::NotEqual{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::NotEqual{start,end,inner}
            },
            raw::Expr::GreaterEqual{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::GreaterEqual{start,end,inner}
            },
            raw::Expr::LessEqual{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::LessEqual{start,end,inner}
            },
            raw::Expr::Greater{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Greater{start,end,inner}
            },
            raw::Expr::Less{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Less{start,end,inner}
            },
            raw::Expr::And{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::And{start,end,inner}
            },
            raw::Expr::Or{start,end,inner}=>{
                let [inner0,inner1]=*inner;
                let inner=Box::new([inner0.convert(scopes,parent,filename)?,inner1.convert(scopes,parent,filename)?]);
                Expr::Or{start,end,inner}
            },
            raw::Expr::Not{start,end,inner}=>{
                let inner=Box::new(inner.convert(scopes,parent,filename)?);
                Expr::Not{start,end,inner}
            },
            raw::Expr::IsType{start,end,inner,ty}=>{
                let to_test=Box::new(inner.convert(scopes,parent,filename)?);
                let ty=ty.convert(scopes,parent,filename)?;
                Expr::IsType{start,end,to_test,ty}
            },
            raw::Expr::ObjectCreation{start,end,inner}=>{
                let mut fields=Vec::new();
                for field in inner {
                    fields.push(field.convert(scopes,parent,filename)?);
                }
                Expr::ObjectCreation{start,end,fields}
            },
            raw::Expr::AnonFunction{inner:raw::AnonFunction{start,end,params,ret_type,block},..}=>{
                let ret_type=ret_type.convert(scopes,parent,filename)?;
                let params=params.convert(scopes,parent,filename)?;
                let scope=scopes.push(VarScopeItem::AnonFunction {
                    imports:Vec::new(),
                    parent_scope:parent,
                    def_start:start,
                    def_end:end,
                    params:Vec::new(),
                    ret_type,
                    statements:Vec::new(),
                    vars:HashMap::new(),
                });
                for param in params.normal.into_iter() {
                    let name=param.name;
                    if scopes.get_mut(scope).unwrap().get_var(param.name).is_some() {
                        return Err(Error::new_verif(filename,start,end,format!("parameter {} is specified twice",name)));
                    }
                    let param_scope=scopes.push(VarScopeItem::Parameter {
                        parent_scope:parent,
                        def_start:param.start,
                        def_end:param.end,
                        mutable:param.mutable,
                        name,
                        ty:param.ty,
                    });
                    scopes.get_mut(scope).unwrap().add_var(name,param_scope);
                }
                scopes.get_mut(parent).expect("Internal compiler error: invalid scope").add_stmt(Statement::FunctionDef{start,end,def:scope});
                for s in block.inner {
                    s.convert(scopes,scope,filename)?;
                }
                Expr::AnonFunction{start,end,function:scope}
            },
            raw::Expr::Ref{start,end,inner}=>{
                todo!();
            },
            raw::Expr::RefMut{start,end,inner}=>{
                todo!();
            },
            raw::Expr::ForeverLoop{start,end,inner}=>{
                todo!();
            },
            raw::Expr::WhileLoop{start,end,condition,block}=>{
                todo!();
            },
            raw::Expr::ForLoop{start,end,var,iterator,block}=>{
                todo!();
            },
            raw::Expr::Match{start,end,inner}=>{
                todo!();
            },
        })
    }
}
impl<'input> Convert<'input> for raw::Block<'input> {
    type Output=Scope;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let scope=scopes.push(VarScopeItem::Block {
            imports:Vec::new(),
            parent_scope:parent,
            def_start:self.start,
            def_end:self.end,
            statements:Vec::new(),
            vars:Default::default(),
        });
        for s in self.inner.into_iter() {
            s.convert(scopes,scope,filename)?;
        }
        return Ok(scope);
    }
}
impl<'input> Convert<'input> for raw::Data<'input> {
    type Output=Data<'input>;
    fn convert(self,_:&mut Scopes<'input>,_:Scope,_:&'input str)->Result<Self::Output,Error<'input,String>> {
        Ok(match self {
            raw::Data::String{start,end,inner}=>Data::String{start,end,s:inner},
            raw::Data::GenericNumber{start,end,negative,inner}=>Data::GenericNumber{start,end,negative,data:inner},
            raw::Data::GenericFloat{start,end,negative,inner}=>Data::GenericFloat{start,end,negative,data:inner},
            raw::Data::Char{start,end,inner}=>Data::Char{start,end,data:inner},
            raw::Data::Bool{start,end,inner}=>Data::Bool{start,end,data:inner},
            _=>unreachable!(),
        })
    }
}
impl<'input> Convert<'input> for raw::ObjectField<'input> {
    type Output=ObjectField<'input>;
    fn convert(self,scopes:&mut Scopes<'input>,parent:Scope,filename:&'input str)->Result<Self::Output,Error<'input,String>> {
        let raw::ObjectField{start,end,public,mutable,name,data}=self;
        let public=public.convert(scopes,parent,filename)?;
        let mutable=mutable.convert(scopes,parent,filename)?;
        let data=data.convert(scopes,parent,filename)?;
        return Ok(ObjectField{start,end,public,mutable,name,data});
    }
}


pub fn refine<'input>(statements:Vec<raw::Statement<'input>>,filename:&'input str)->Result<Scopes<'input>,Error<'input,String>> {
    let mut scopes=Scopes::default();
    let root_scope=scopes.push(VarScopeItem::Root{modules:Vec::new(),imports:Vec::new(),statements:Vec::new(),vars:Default::default()});
    for s in statements {
        s.convert(&mut scopes,root_scope,filename)?;
    }
    return Ok(scopes);
}
