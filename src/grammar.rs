use std::{
    cmp::Ordering as CmpOrdering,
    collections::HashMap,
    rc::Rc,
    sync::Arc,
    mem::swap,
};
use crate::Error;
pub use object_lang::*;


#[derive(Debug)]
pub enum ConstraintError {
    CannotSubtract,
    CannotIntersect,
    DoesNotExist,
    Other(Error),
}
impl From<Error> for ConstraintError {
    fn from(e:Error)->Self {Self::Other(e)}
}
#[derive(Debug)]
pub enum Statement {
    Module {
        name:String,
    },
    Delete {
        path:Vec<String>,
    },
    Ref {
        of:BlockOrPath,
    },
    AtomicRef {
        of:BlockOrPath,
    },
    Return {
        value:Option<BlockOrPath>,
    },
    Import {
        path:ObjectOrConstraintPath,
        rename:Option<String>,
    },
    Variable {
        public:bool,
        mutable:MutableMode,
        name:String,
        constraint:Option<ConstraintOrPath>,
        data:Option<Box<Self>>,
    },
    Constraint {
        name:String,
        generics:Vec<String>,
        first:ConstraintOrPath,
        modifications:Vec<(ConstraintOrPath,ConstraintOperation)>,
    },
    Eval {
        data:BlockOrPath,
        args:Option<Box<Statement>>,
    },
    New {
        path:Vec<String>,
    },
    Lazy {
        parameters:ConstraintOrPath,
        ret_val:ConstraintOrPath,
        block:Vec<Self>,
    },
    String {
        data:String,
    },
    Block(Vec<Self>),
}
#[derive(PartialEq,Clone,Debug)]
pub enum GenericOrConsPath {
    Generic(String),
    ConsPath(Box<ConstraintOrPath>),
}
#[derive(Debug)]
pub enum BlockOrPath {
    Block(Vec<Statement>),
    Path(Vec<String>),
}
#[derive(Debug)]
pub enum ObjectOrConstraintPath {
    Object(Vec<String>),
    Constraint(Vec<String>,String),
}
#[derive(Debug)]
pub enum Object {
    Object(HashMap<String,ObjectField>),
    Slice(Vec<Self>),
    Ref(Rc<Self>),
    AtomicRef(Arc<Self>),
    SingleFloat(f32),
    DoubleFloat(f64),
    Byte(i8),
    UByte(u8),
    Int(i32),
    UInt(u32),
    LongInt(i64),
    LongUInt(u64),
    Bool(bool),
    Function(String),
    None,
}
impl Object {
    pub fn get_constraint(&self,module:&ModuleObject,functions:&HashMap<String,Constraint>)->Result<Constraint,ConstraintError> {
        return Ok(match self {
            Object::Function(name)=>functions.get(name).ok_or(ConstraintError::DoesNotExist)?.clone(),
            Object::Object(fields)=>{
                let mut constraint=HashMap::new();
                for (name,data) in fields.iter() {
                    constraint.insert(name.clone(),data.get_constraint(module,functions)?);
                }
                Constraint::Object(constraint)
            },
            Object::Slice(items)=>{
                let mut constraint=Constraint::Object(HashMap::new());
                for item in items.iter() {
                    let item_constraint=item.get_constraint(module,functions)?;
                    constraint.intersection(&item_constraint,module)?;
                }
                Constraint::Slice(Box::new(constraint.into()))
            },
            Object::Ref(r)=>Constraint::Ref(Box::new(r.get_constraint(module,functions)?.into())),
            Object::AtomicRef(r)=>Constraint::AtomicRef(Box::new(r.get_constraint(module,functions)?.into())),
            Object::SingleFloat(_)=>Constraint::SingleFloat,
            Object::DoubleFloat(_)=>Constraint::DoubleFloat,
            Object::Byte(_)=>Constraint::Byte,
            Object::UByte(_)=>Constraint::UByte,
            Object::Int(_)=>Constraint::Int,
            Object::UInt(_)=>Constraint::UInt,
            Object::LongInt(_)=>Constraint::LongInt,
            Object::LongUInt(_)=>Constraint::LongUInt,
            Object::Bool(_)=>Constraint::Bool,
            Object::None=>Constraint::None,
        });
    }
}
#[derive(PartialEq,Clone,Debug)]
pub enum ConstraintOrPath {
    Constraint(Constraint),
    Path(Vec<String>,String,Option<Vec<GenericOrConsPath>>),
}
impl From<Constraint> for ConstraintOrPath {
    fn from(c:Constraint)->Self {
        Self::Constraint(c)
    }
}
impl ConstraintOrPath {
    pub fn is_satisfied(&self,generics:&HashMap<String,Constraint>,module:&ModuleObject,core_constraints:&HashMap<&'static str,Constraint>,obj:&Object)->Result<bool,Error> {
        match self {
            Self::Constraint(c)=>c.is_satisfied(generics,module,core_constraints,obj),
            Self::Path(path,name,_)=>{
                let mut path_string=String::new();
                path.iter().for_each(|p|{path_string.push_str(p);path_string.push('.')});
                path_string.pop();
                return module.get_constraint(path,name)
                    .ok_or(Error::UnknownConstraint(format!("{}::{}",path_string,name)))?
                    .is_satisfied(generics,module,core_constraints,obj);
            },
        }
    }
    pub fn sub(&mut self,module:&ModuleObject,other:&Constraint)->Result<(),ConstraintError> {
        self.collapse(module)?;
        match self {
            Self::Constraint(c)=>{
                c.sub(module,other)?;
            },
            _=>unreachable!(),
        }
        return Ok(());
    }
    pub fn intersection(&mut self,other:&Self,module:&ModuleObject)->Result<(),ConstraintError> {
        self.collapse(module)?;
        match self {
            Self::Constraint(c)=>c.intersection(other.collapse_no_check()?,module)?,
            _=>unreachable!(),
        }
        return Ok(());
    }
    #[inline]
    pub fn is_nothing(&self)->bool {
        match self {
            Self::Constraint(c)=>c.is_nothing(),
            _=>false,
        }
    }
    pub fn collapse_no_check(&self)->Result<&Constraint,Error> {
        match self {
            Self::Constraint(c)=>Ok(c),
            _=>Err(Error::RequestPathCollapse),
        }
    }
    pub fn collapse(&mut self,module:&ModuleObject)->Result<&Constraint,Error> {
        match self {
            Self::Constraint(c)=>Ok(c),
            Self::Path(path,name,_)=>{
                let mut path_string=String::new();
                path.iter().for_each(|p|{path_string.push_str(p);path_string.push('.')});
                path_string.pop();
                let c=module.get_constraint(path,name)
                    .ok_or(Error::UnknownConstraint(format!("{}::{}",path_string,name)))?;
                *self=Self::Constraint(c.clone());
                return self.collapse(module);
            },
        }
    }
    pub fn collapse_mut(&mut self,module:&ModuleObject)->Result<&mut Constraint,Error> {
        self.collapse(module)?;
        match self {
            Self::Constraint(c)=>Ok(c),
            _=>unreachable!(),
        }
    }
}
#[derive(PartialEq,Clone,Debug)]
pub enum Constraint {
    Object(HashMap<String,ConstraintField>),
    Slice(Box<ConstraintOrPath>),
    Ref(Box<ConstraintOrPath>),
    AtomicRef(Box<ConstraintOrPath>),
    Function(Box<ConstraintOrPath>,Box<ConstraintOrPath>),
    SingleFloat,
    DoubleFloat,
    Byte,
    UByte,
    Int,
    UInt,
    LongInt,
    LongUInt,
    Bool,
    None,
    Generic(String),
    // Special compiler-only constraints
    Either(Vec<ConstraintOrPath>),
    And(Vec<ConstraintOrPath>),
    /// Different from `None` because this does not match with anything. Not even itself.
    Nothing,
}
impl Constraint {
    pub fn is_satisfied(&self,generics:&HashMap<String,Constraint>,module:&ModuleObject,core_constraints:&HashMap<&'static str,Self>,obj:&Object)->Result<bool,Error> {
        match self {
            Self::Object(fields)=>{
                // Special handling of the `Any` constraint
                if fields.len()==0 {
                    return Ok(true);
                }
                match obj {
                    Object::Object(obj_fields)=>{
                        for (key,constraint) in fields.iter() {
                            if let Some(obj_field)=obj_fields.get(key) {
                                if !constraint.is_satisfied(generics,module,core_constraints,obj_field)? {
                                    return Ok(false);
                                }
                            } else {
                                return Ok(false);
                            }
                        }
                        return Ok(true);
                    },
                    _=>return Ok(false),
                }
            },
            Self::Slice(c)=>{
                match obj {
                    Object::Slice(list)=>{
                        for item in list {
                            if !c.is_satisfied(generics,module,core_constraints,item)? {
                                return Ok(false);
                            }
                        }
                        return Ok(true);
                    },
                    _=>{},
                }
            },
            Self::Ref(c)=>{
                match obj {
                    Object::Ref(obj)=>{
                        return c.is_satisfied(generics,module,core_constraints,obj);
                    },
                    _=>{},
                }
            },
            Self::AtomicRef(c)=>{
                match obj {
                    Object::AtomicRef(obj)=>{
                        return c.is_satisfied(generics,module,core_constraints,obj);
                    },
                    _=>{},
                }
            },
            Self::Either(constraints)=>{
                for constraint in constraints.iter() {
                    if constraint.is_satisfied(generics,module,core_constraints,obj)? {
                        return Ok(true);
                    }
                }
                return Ok(false);
            },
            a=>{
                match a {
                    Self::SingleFloat=>{
                        let constraint=core_constraints.get("SingleFloat").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::DoubleFloat=>{
                        let constraint=core_constraints.get("DoubleFloat").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::Byte=>{
                        let constraint=core_constraints.get("Byte").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::UByte=>{
                        let constraint=core_constraints.get("UByte").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::Int=>{
                        let constraint=core_constraints.get("Int").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::UInt=>{
                        let constraint=core_constraints.get("UInt").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::LongInt=>{
                        let constraint=core_constraints.get("LongInt").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::LongUInt=>{
                        let constraint=core_constraints.get("LongUInt").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    Self::Bool=>{
                        let constraint=core_constraints.get("Bool").expect("Compiler error #");
                        if let Ok(true)=constraint.is_satisfied(generics,module,core_constraints,obj) {
                            return Ok(true);
                        }
                    },
                    _=>{},
                }
                match (a,obj) {
                    (Self::SingleFloat,Object::SingleFloat(_))|
                        (Self::DoubleFloat,Object::DoubleFloat(_))|
                        (Self::Byte,Object::Byte(_))|
                        (Self::UByte,Object::UByte(_))|
                        (Self::Int,Object::Int(_))|
                        (Self::UInt,Object::UInt(_))|
                        (Self::LongInt,Object::LongInt(_))|
                        (Self::LongUInt,Object::LongUInt(_))|
                        (Self::Bool,Object::Bool(_))|
                        (Self::None,Object::None)=>return Ok(true),
                    _=>{},
                }
            },
        }
        return Ok(false);
    }
    #[inline]
    pub fn is_nothing(&self)->bool {
        match self {
            Self::Nothing=>true,
            _=>false,
        }
    }
    pub fn sub(&mut self,module:&ModuleObject,other:&Self)->Result<(),ConstraintError> {
        match self {
            Self::Object(fields)=>{
                match other {
                    Self::Object(fields2)=>{
                        for (key,c) in fields2.iter() {
                            if let Some(c1)=fields.get_mut(key) {
                                c1.sub(module,c)?;
                                if c1.is_nothing() {fields.remove(key);}
                            }
                        }
                    },
                    _=>return Err(ConstraintError::CannotSubtract),
                }
            },
            a=>if a==other {*a=Self::Nothing} else {return Err(ConstraintError::CannotSubtract)},
        }
        return Ok(());
    }
    pub fn add(&mut self,other:Self) {
        match self {
            Constraint::Either(constraints)=>constraints.push(other.into()),
            _=>{
                let mut new_constraint=Constraint::Either(vec![other.into()]);
                swap(self,&mut new_constraint);
                self.either(new_constraint);
            },
        }
    }
    pub fn intersection(&mut self,other:&Self,module:&ModuleObject)->Result<(),ConstraintError> {
        use Constraint::*;
        match (self,other) {
            (Object(fields1),Object(fields2))=>{
                for (k,c) in fields1.iter_mut() {
                    if let Some(c2)=fields2.get(k) {
                        c.intersection(c2,module)?;
                    }
                }
                fields1.retain(|_,c|!c.is_nothing());
            },
            (Slice(a),Slice(b))=>a.intersection(b,module)?,
            (Ref(a),Ref(b))=>a.intersection(b,module)?,
            (AtomicRef(a),AtomicRef(b))=>a.intersection(b,module)?,
            (Either(constraints),other)=>{
                for c in constraints.iter_mut() {
                    let c=c.collapse_mut(module)?;
                    c.intersection(other,module)?
                }
            },
            (a,Either(constraints))=>{
                let mut list=Vec::new();
                for constraint in constraints.iter() {
                    let mut new_constraint=a.clone();
                    new_constraint.intersection(constraint.collapse_no_check()?,module)?;
                    list.push(new_constraint.into());
                }
                *a=Constraint::Either(list);
            },
            (SingleFloat,SingleFloat)=>{},
            (DoubleFloat,DoubleFloat)=>{},
            (Byte,Byte)=>{},
            (UByte,UByte)=>{},
            (Int,Int)=>{},
            (UInt,UInt)=>{},
            (LongInt,LongInt)=>{},
            (LongUInt,LongUInt)=>{},
            (None,None)=>{},
            (Nothing,_)=>{},
            (a,b)=>if a!=b {
                return Err(ConstraintError::CannotIntersect);
            },
        }
        return Ok(());
    }
    pub fn either(&mut self,other:Self) {
        match self {
            Constraint::Either(constraints)=>constraints.push(other.into()),
            _=>{
                let mut new_constraint=Constraint::Either(vec![other.into()]);
                swap(self,&mut new_constraint);
                self.either(new_constraint);
            },
        }
    }
}
#[derive(PartialEq,Clone,Copy,Debug)]
pub enum MutableMode {
    PublicMutable,
    SelfMutable,
    None,
}
impl PartialOrd for MutableMode {
    fn partial_cmp(&self,other:&Self)->Option<CmpOrdering> {
        use MutableMode::*;
        match (self,other) {
            (PublicMutable,SelfMutable)|
                (PublicMutable,None)|
                (SelfMutable,None)=>Some(CmpOrdering::Greater),
            (SelfMutable,PublicMutable)|
                (None,PublicMutable)|
                (None,SelfMutable)=>Some(CmpOrdering::Less),
            _=>Some(CmpOrdering::Equal),
        }
    }
}
impl MutableMode {
    pub fn intersection(&mut self,other:&Self) {
        if *self>*other {
            *self=*other;
        }
    }
    pub fn sub(&mut self,other:&Self) {
        use MutableMode::*;
        *self=match (&self,other) {
            (PublicMutable,PublicMutable)=>SelfMutable,
            (SelfMutable,PublicMutable)=>SelfMutable,
            _=>None,
        };
    }
}
#[derive(Debug)]
pub enum ConstraintOperation {
    Intersection,
    Either,
    Add,
    Sub,
}


#[derive(PartialEq,Clone,Debug)]
pub struct ConstraintField {
    pub mutable:MutableMode,
    pub public:bool,
    pub constraint:ConstraintOrPath,
}
impl ConstraintField {
    pub fn is_satisfied(&self,generics:&HashMap<String,Constraint>,module:&ModuleObject,core_constraints:&HashMap<&'static str,Constraint>,obj:&ObjectField)->Result<bool,Error> {
        Ok(
            obj.mutable==self.mutable&&
            obj.public==self.public&&
            self.constraint.is_satisfied(generics,module,core_constraints,&obj.object)?
        )
    }
    pub fn sub(&mut self,module:&ModuleObject,other:&Self)->Result<(),ConstraintError> {
        self.mutable.sub(&other.mutable);
        if other.public {
            self.public=false;
        }
        return self.constraint.sub(module,other.constraint.collapse_no_check()?);
    }
    pub fn intersection(&mut self,other:&Self,module:&ModuleObject)->Result<(),ConstraintError> {
        self.public&=other.public;
        self.mutable.intersection(&other.mutable);
        return self.constraint.intersection(&other.constraint,module);
    }
    #[inline]
    pub fn is_nothing(&self)->bool {self.constraint.is_nothing()}
}
#[derive(Debug)]
pub struct ObjectField {
    pub mutable:MutableMode,
    pub public:bool,
    pub object:Object,
}
impl ObjectField {
    pub fn get_constraint(&self,module:&ModuleObject,functions:&HashMap<String,Constraint>)->Result<ConstraintField,ConstraintError> {
        return Ok(ConstraintField {
            mutable:self.mutable,
            public:self.public,
            constraint:self.object.get_constraint(module,functions)?.into(),
        });
    }
}
#[derive(Debug)]
pub struct Import {
    pub path:Vec<String>,
    pub name:String,
    pub rename:Option<String>,
}
impl Import {
    pub fn new(mut path:Vec<String>,rename:Option<String>)->Self {
        let name=path.pop().expect("Path did not contain any items");
        Self {name,path,rename}
    }
    pub fn name(&self)->&String {
        if let Some(name)=&self.rename {
            return &name;
        }
        return &self.name;
    }
}
#[derive(Debug)]
pub struct ModuleObject {
    pub constraint_imports:Vec<Import>,
    pub constraints:HashMap<String,(Vec<String>,Constraint)>,
    pub object_imports:Vec<Import>,
    pub statements:Vec<Statement>,
    pub submodules:HashMap<String,ModuleObject>,
}
impl ModuleObject {
    pub fn new(mut statements:Vec<Statement>)->(Self,Vec<String>) {
        let mut constraint_imports=Vec::new();
        let constraints=HashMap::new();
        let mut object_imports=Vec::new();
        let mut needed_modules=Vec::new();
        statements.retain(|s|{
            match s {
                Statement::Import{path,rename}=>{
                    match path {
                        ObjectOrConstraintPath::Object(path)=>constraint_imports.push(Import::new(path.clone(),rename.clone())),
                        ObjectOrConstraintPath::Constraint(path,name)=>object_imports.push(Import{path:path.clone(),name:name.clone(),rename:rename.clone()}),
                    }
                    false
                },
                Statement::Module{name}=>{
                    needed_modules.push(name.clone());
                    false
                },
                _=>true,
            }
        });
        return (
            Self {
                constraint_imports,
                constraints,
                object_imports,
                submodules:HashMap::new(),
                statements,
            },
            needed_modules,
        );
    }
    pub fn finish_parsing(&mut self)->Result<(),ConstraintError> {
        let mut old_statements=Vec::new();
        swap(&mut old_statements,&mut self.statements);
        self.statements=old_statements
            .into_iter()
            // kinda a hack, but filter_map does what I need to do.
            .filter_map(|statement|{
                match statement {
                    Statement::Constraint{name,mut first,generics,modifications}=>{
                        // TODO: proper error handling here
                        first.collapse(self).unwrap();
                        match first {
                            ConstraintOrPath::Constraint(mut c)=>{
                                for (mut cons,op) in modifications {
                                    match op {
                                        ConstraintOperation::Intersection=>{
                                            c.intersection(cons.collapse(self).unwrap(),self).unwrap();
                                        },
                                        ConstraintOperation::Either=>{
                                            c.either(cons.collapse(self).unwrap().clone());
                                        },
                                        ConstraintOperation::Add=>{
                                            c.add(cons.collapse(self).unwrap().clone());
                                        },
                                        ConstraintOperation::Sub=>{
                                            c.sub(self,cons.collapse(self).unwrap()).unwrap();
                                        },
                                    }
                                }
                                self.constraints.insert(name,(generics,c));
                            },
                            _=>unreachable!(),
                        }
                        None
                    },
                    s=>Some(s),
                }
            })
            .collect();
        return Ok(());
    }
    pub fn get_constraint<'a>(&'a self,mut path:&'a [String],mut name:&'a String)->Option<&'a Constraint> {
        if path.len()>0 {
            if let Some(c)=self.constraints.get(name) {
                return Some(&c.1);
            } else {
                if let Some(ci)=self.constraint_imports.iter().find(|ci|ci.name()==name) {
                    path=ci.path.as_slice();
                    name=&ci.name;
                } else {
                    return None;
                }
            }
        }
        return self.submodules.get(&path[0])?.get_constraint(&path[1..],name);
    }
}


peg::parser!(grammar object_lang() for str {
    pub rule parse_module()->Vec<Statement>=
        quiet!{comment()*}
        items:parse_statement_list()
        quiet!{comment()*}
        ![_]
        {items}
    rule parse_block_list()->Vec<Statement>=
        "{"
        _ __ _
        statements:parse_statement_list()
        _ __ _
        "}"
        {statements}
    rule parse_block()->Statement=
        list:parse_block_list()
        {Statement::Block(list)}
    rule parse_statement_list()->Vec<Statement>=
        _ __ _
        items:parse_statement()**(_ ___* _ __ _)
        _ __ _
        {items}
    rule parse_statement()->Statement=(
        parse_block()/
        parse_constraint_statement()/
        parse_var()/
        parse_eval()/
        parse_new()/
        parse_lazy()/
        parse_module_import()/
        parse_delete()/
        parse_return()/
        parse_import()/
        parse_ref()/
        parse_atomic_ref()/
        parse_string()
    )
    rule parse_module_import()->Statement=
        "module"
        _
        name:parse_name()
        {Statement::Module{name}}
    rule parse_delete()->Statement=
        "delete"
        _
        path:parse_object_path()
        {Statement::Delete{path}}
    rule parse_import()->Statement=
        "import"
        _
        path:parse_constraint_or_object_path()
        _
        rename:(
            "as"
            _
            name:parse_name()
            {name}
        )?
        {Statement::Import{path,rename}}
    rule parse_constraint_or_object_path()->ObjectOrConstraintPath=
        (
            c:parse_constraint_path()
            {ObjectOrConstraintPath::Constraint(c.0,c.1)}
        )/(
            o:parse_object_path()
            {ObjectOrConstraintPath::Object(o)}
        )
    rule parse_constraint_path()->(Vec<String>,String)=
        path:parse_name()**(_ __ _ "." _ __ _)
        _ __ _
        "::"
        _ __ _
        name:parse_name()
        {(path,name)}
    rule parse_return()->Statement=
        "return"
        _
        value:parse_block_or_path()?
        {Statement::Return{value}}
    rule parse_ref()->Statement=
        "ref"
        _
        of:parse_block_or_path()
        {Statement::Ref{of}}
    rule parse_atomic_ref()->Statement=
        "atomic"
        _
        "ref"
        _
        of:parse_block_or_path()
        {Statement::AtomicRef{of}}
    rule parse_string()->Statement=
        "\""
        data:(!"\"" c:[_]{c})*
        "\""
        {Statement::String{data:data.into_iter().collect()}}
    rule parse_lazy()->Statement=
        "lazy"
        "["
        _
        parameters:parse_constraint_or_path()
        _
        ","
        _
        ret_val:parse_constraint_or_path()
        _
        "]"
        _
        block:parse_block_list()
        {Statement::Lazy{block,parameters,ret_val}}
    rule parse_constraint_statement()->Statement=
        "constraint"
        _
        name:parse_name()
        generics:(
            "("
            names:("~" name:parse_name(){name})*
            ")"
            {names}
        )?
        _
        "="
        _
        first:parse_constraint_or_path()
        modifications:(
            _ __ _
            op:(
                ("^"{ConstraintOperation::Intersection})/
                ("|"{ConstraintOperation::Either})/
                ("+"{ConstraintOperation::Add})/
                ("-"{ConstraintOperation::Sub})
            )
            _ __ _
            c:parse_constraint_or_path()
            {(c,op)}
        )*
        {Statement::Constraint{name,generics:generics.unwrap_or(Vec::new()),first,modifications}}
    rule parse_constraint_or_path()->ConstraintOrPath=
        (
            c:parse_constraint_block()
            {ConstraintOrPath::Constraint(c)}
        )/(
            c:parse_constraint()
            {ConstraintOrPath::Constraint(c)}
        )/(
            name:parse_name()
            generics:(
                "("
                generics:(
                    (
                        "~"
                        name:parse_name()
                        {GenericOrConsPath::Generic(name)}
                    )/(
                        path:parse_constraint_or_path()
                        {GenericOrConsPath::ConsPath(Box::new(path))}
                    )
                )**(_ __ _ "," _ __ _)
                ")"
                {generics}
            )?
            {ConstraintOrPath::Path(Vec::new(),name,generics)}
        )/(
            names:parse_name()**(_ "." _)
            _
            "::"
            _
            name:parse_name()
            generics:(
                "("
                generics:(
                    (
                        "~"
                        name:parse_name()
                        {GenericOrConsPath::Generic(name)}
                    )/(
                        path:parse_constraint_or_path()
                        {GenericOrConsPath::ConsPath(Box::new(path))}
                    )
                )**(_ __ _ "," _ __ _)
                ")"
                {generics}
            )?
            {ConstraintOrPath::Path(names,name,generics)}
        )
    rule parse_public()->bool=
        (
            "public"
            {true}
        )/(
            {false}
        )
    rule parse_mutable()->MutableMode=
        (
            "mutable[self]"
            {MutableMode::SelfMutable}
        )/(
            "mutable"
            {MutableMode::PublicMutable}
        )/(
            {MutableMode::None}
        )
    rule parse_constraint_block()->Constraint=
        "<"
        items:(
            _ __ _
            public:parse_public()
            _
            mutable:parse_mutable()
            _
            name:parse_name()
            _
            ":"
            _
            constraint:parse_constraint_or_path()
            {(name.to_string(),ConstraintField{public,mutable,constraint})}
        )**(_ "," _ __ _)
        _ __ _
        ","?
        _ __ _
        ">"
        {Constraint::Object(items.into_iter().collect())}
    rule parse_constraint()->Constraint=
        ("~" name:parse_name(){Constraint::Generic(name)})/
        (
            "Fn("
            _
            parameters:parse_constraint_or_path()
            _
            ","
            _
            ret_type:parse_constraint_or_path()
            _
            ")"
            {Constraint::Function(Box::new(parameters),Box::new(ret_type))}
        )/
        (
            name:parse_name()
            {?
                match name.as_str() {
                    "None"=>Ok(Constraint::Byte),
                    "Byte"=>Ok(Constraint::Byte),
                    "UByte"=>Ok(Constraint::Byte),
                    "Bool"=>Ok(Constraint::Byte),
                    "Int"=>Ok(Constraint::Byte),
                    "UInt"=>Ok(Constraint::Byte),
                    "LongInt"=>Ok(Constraint::Byte),
                    "LongUInt"=>Ok(Constraint::Byte),
                    "SingleFloat"=>Ok(Constraint::Byte),
                    "DoubleFloat"=>Ok(Constraint::Byte),
                    _=>Err("Expected immutable type"),
                }
            }
        )/
        ("ref " _ c:parse_constraint_or_path(){Constraint::Ref(Box::new(c))})/
        ("atomic " _ "ref " _ c:parse_constraint_or_path(){Constraint::AtomicRef(Box::new(c))})
    rule parse_object_path()->Vec<String>=
        parse_name()**(_ "." _)
    rule parse_new()->Statement=
        "new"
        _
        path:parse_object_path()
        {Statement::New{path}}
    rule parse_var()->Statement=
        public:parse_public()
        _
        mutable:parse_mutable()
        _
        "var"
        _
        name:parse_name()
        _
        constraint:(":" _ c:parse_constraint_or_path(){c})?
        _
        "="
        _
        data:parse_statement()?
        {Statement::Variable{public,mutable,name,constraint,data:data.map(Box::new)}}
    rule parse_block_or_path()->BlockOrPath=
        (
            list:parse_block_list()
            {BlockOrPath::Block(list)}
        )/(
            p:parse_object_path()
            {BlockOrPath::Path(p)}
        )
    rule parse_eval()->Statement=
        "eval"
        args:(
            "["
            _
            o:parse_statement()
            _
            "]"
            {o}
        )?
        _
        data:parse_block_or_path()
        {Statement::Eval{args:args.map(Box::new),data}}
    /// parse everything that isn't one of the name endings
    rule parse_name()->String=
        chars:(!name_endings() c:[_]{c})+
        {chars.into_iter().collect()}
    rule name_endings()=['.'|'('|')'|' '|'\n'|'='|':'|'~'|'+'|'^'|'-'|'|'|','|'<'|'>'|'['|']']
    /// whitespace
    rule _()=(
        comment()+/
        [' '|'\t']+
    )*
    /// newlines
    rule __()=(
        comment()+/
        newline()+
    )*
    /// statement terminator
    rule ___()=['\n'|';']
    rule newline()=(['\n'|'\r']/"\r\n")
    rule comment()=(
        (
            "/*"
            (!"*/" (&"/*" comment())? [_])*
            "*/"
        )/(
            "//"
            (!newline() [_])*
            newline()
        )
    )
});
