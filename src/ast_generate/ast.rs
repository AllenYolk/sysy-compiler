#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

/// FuncDef ::= FuncType IDENT "(" ")" Block;
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
    Void,
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
///        | "return" [Exp] ";";
#[derive(Debug)]
pub enum Stmt {
    Assign(LVal, Exp),
    Exp(Option<Exp>),
    Block(Block),
    If(Exp, Box<Stmt>, Option<Box<Stmt>>),
    Return(Option<Exp>),
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
    pub init: ConstInitVal,
}

#[derive(Debug)]
pub enum ConstInitVal {
    Exp(ConstExp),
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
    pub init: Option<InitVal>,
}

#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
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

#[derive(Debug)]
pub enum UnaryExp {
    Primary(PrimaryExp),
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
}

////////////////////////////////////////////////////////////////////////////
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
