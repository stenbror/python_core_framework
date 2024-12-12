use std::vec::Vec;

#[derive(PartialEq)]
enum Token
{
    Empty
}


#[derive(PartialEq)]
enum ASTNode
{
    Empty,

    /* Expression nodes */
    NamedExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    TestExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    LambdaExpr(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>, Box<ASTNode>),
    OrTest(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    AndTest(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    NotTest(u32, u32, Box<Token>, Box<ASTNode>),
    CompareLess(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareGreater(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareEqual(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareGreaterEqual(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareLessEqual(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareNotEqual(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareIn(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareNotIn(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    CompareIs(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    CompareIsNot(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    StarExpr(u32, u32, Box<Token>, Box<ASTNode>),
    Expr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    XorExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    AndExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ShiftLeft(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ShiftRight(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BinaryOperatorPlus(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BinaryOperatorMinus(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BinaryOperatorMul(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BinaryOperatorMatrice(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BinaryOperatorDiv(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BinaryOperatorModulo(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BinaryOperatorFloorDiv(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    UnaryOperatorPlus(u32, u32, Box<Token>, Box<ASTNode>),
    UnaryOperatorMinus(u32, u32, Box<Token>, Box<ASTNode>),
    UnaryOperatorInvert(u32, u32, Box<Token>, Box<ASTNode>),
    BinaryOperatorPower(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    AtomExpr(u32, u32, Option<Box<Token>>, Box<ASTNode>, Box<Vec::<Box<ASTNode>>>),
    LiteralName(u32, u32, Box<Token>),
    LiteralNumber(u32, u32, Box<Token>),
    LiteralString(u32, u32, Box<Vec::<Box<str>>>),
    LiteralElipsis(u32, u32, Box<Token>),
    LiteralNone(u32, u32, Box<Token>),
    LiteralTrue(u32, u32, Box<Token>),
    LiteralFalse(u32, u32, Box<Token>),
    LiteralTuple(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>),
    LiteralList(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>),
    LiteralDictionary(u32, u32, Box<Token>, Option<Box<( Box<Vec::<Box<ASTNode>>>, Box<Vec<Box<Token>>> )>>, Box<Token>),
    DictionaryEntry(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    DictionaryReference(u32, u32, Box<Token>, Box<ASTNode>),
    LiteralSet(u32, u32, Box<Token>, Option<Box<( Box<Vec::<Box<ASTNode>>>, Box<Vec<Box<Token>>> )>>, Box<Token>),
    LiteralSetReference(u32, u32, Box<Token>, Box<ASTNode>),
    TestListComp(u32, u32, Box<Vec::<Box<ASTNode>>>, Box<Vec::<Box<Token>>>),
    TrailerDotName(u32, u32, Box<Token>, Box<Token>),
    TrailerCall(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>),
    TrailerIndex(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>),
    SubscriptList(u32, u32, Box<Vec::<Box<ASTNode>>>, Box<Vec::<Box<Token>>>),
    Subscript(u32, u32, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>),
    ExprList(u32, u32, Box<Vec::<Box<ASTNode>>>, Box<Vec::<Box<Token>>>),
    TestList(u32, u32, Box<Vec::<Box<ASTNode>>>, Box<Vec::<Box<Token>>>),
    ArgList(u32, u32, Box<Vec::<Box<ASTNode>>>, Box<Vec::<Box<Token>>>),
    Argument(u32, u32, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>),
    SyncCompFor(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>, Option<Box<ASTNode>>),
    CompFor(u32, u32, Box<Token>, Box<ASTNode>),
    CompIf(u32, u32, Box<Token>, Box<ASTNode>, Option<Box<ASTNode>>),
    YieldExpr(u32, u32, Box<Token>, Option<Box<ASTNode>>),
    YieldFromExpr(u32, u32, Box<Token>, Box<Token>, Box<ASTNode>)
}
