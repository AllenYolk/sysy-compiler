/// CompUnit ::= [CompUnit] (GlobalDecl | FuncDef);
/// GlobalDecl ::= Decl;
#[derive(Debug)]
pub struct CompUnit {
    pub items: Vec<CompUnitItem>,
}

#[derive(Debug)]
pub enum CompUnitItem {
    GlobalDecl(GlobalDecl),
    FuncDef(FuncDef),
}

/// FuncDef ::= FuncType IDENT "(" [FuncFParams] ")" Block;
/// FuncFParams ::= FuncFParam {"," FuncFParam};
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub params: Vec<FuncFParam>,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
    Void,
}

/// FuncFParam ::= BType IDENT [ "[" "]" { "[" ConstExp "]" } ];
/// BType ::= "int"
#[derive(Debug)]
pub struct FuncFParam {
    // there's only `int` type in SysY!
    pub ident: String,
    pub dims: Option<Vec<ConstExp>>,
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Stmt),
    Decl(Decl),
}

/// Stmt ::= LVal "=" Exp ";"
///        | [Exp] ";"
///        | Block
///        | "if" "(" Exp ")" Stmt ["else" Stmt]
///        | "while" "(" Exp ")" Stmt
///        | "break" ";"
///        | "continue" ";"
///        | "return" [Exp] ";";
#[derive(Debug)]
pub enum Stmt {
    Assign(LVal, Exp),
    Exp(Option<Exp>),
    Block(Block),
    If(Exp, Box<Stmt>, Option<Box<Stmt>>),
    While(Exp, Box<Stmt>),
    Break,
    Continue,
    Return(Option<Exp>),
}

#[derive(Debug)]
pub struct GlobalDecl {
    pub decl: Decl,
}

#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl),
}

/// ConstDecl ::= "const" BType ConstDef {"," ConstDef} ";";
#[derive(Debug)]
pub struct ConstDecl {
    // there's only `int` type in SysY!
    pub defs: Vec<ConstDef>,
}

#[derive(Debug)]
pub struct ConstDef {
    pub ident: String,
    pub dims: Vec<ConstExp>,
    pub init: ConstInitVal,
}

#[derive(Debug)]
pub enum ConstInitVal {
    Exp(ConstExp),
    Array(Vec<ConstInitVal>),
}

/// VarDecl ::= BType VarDef {"," VarDef} ";";
#[derive(Debug)]
pub struct VarDecl {
    // there's only `int` type in SysY!
    pub defs: Vec<VarDef>,
}

#[derive(Debug)]
pub struct VarDef {
    pub ident: String,
    pub dims: Vec<ConstExp>,
    pub init: Option<InitVal>,
}

#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
    Array(Vec<InitVal>),
}

////////////////////////////////////////////////////////////////////////////
// Expressions                                                            //
////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ConstExp {
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Exp {
    pub exp: LOrExp,
}

#[derive(Debug)]
pub enum LOrExp {
    LAnd(LAndExp),
    LOrLAnd(Box<LOrExp>, LAndExp),
}

#[derive(Debug)]
pub enum LAndExp {
    Eq(EqExp),
    LAndEq(Box<LAndExp>, EqExp),
}

#[derive(Debug)]
pub enum EqExp {
    Rel(RelExp),
    EqRel(Box<EqExp>, EqExpOp, RelExp),
}

#[derive(Debug)]
pub enum RelExp {
    Add(AddExp),
    RelAdd(Box<RelExp>, RelExpOp, AddExp),
}

#[derive(Debug)]
pub enum AddExp {
    Mul(MulExp),
    AddMul(Box<AddExp>, AddExpOp, MulExp),
}

#[derive(Debug)]
pub enum MulExp {
    Unary(UnaryExp),
    MulUnary(Box<MulExp>, MulExpOp, UnaryExp),
}

/// UnaryExp ::= PrimaryExp
///            | IDENT "(" [FuncRParams] ")"
///            | UnaryOp UnaryExp;
/// FuncRParams ::= Exp {"," Exp};
#[derive(Debug)]
pub enum UnaryExp {
    Primary(PrimaryExp),
    FuncCall(String, Vec<Exp>),
    Unary(UnaryExpOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    LVal(LVal),
    Num(i32),
}

#[derive(Debug)]
pub struct LVal {
    pub ident: String,
    pub idx: Vec<Exp>,
}

///////////////////////////////////////////////////////////////////////////
// Operators                                                             //
///////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum EqExpOp {
    Eq,
    Neq,
}

#[derive(Debug)]
pub enum RelExpOp {
    Le,
    Ge,
    Lt,
    Gt,
}

#[derive(Debug)]
pub enum AddExpOp {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum MulExpOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum UnaryExpOp {
    Pos,
    Neg,
    Not,
}
