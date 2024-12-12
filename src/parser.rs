


#[derive(PartialEq)]
enum ASTNode
{
    Empty,
    BinaryOperatorPlus(u32, u32, Box<ASTNode>, Box<ASTNode>)
}
