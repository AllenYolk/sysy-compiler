#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
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

#[derive(Debug)]
pub struct Stmt {
    pub exp: Exp,
}

#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    //Var(VarDecl),
}

#[derive(Debug)]
pub struct ConstDecl { // there's only `int` type in SysY!
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
