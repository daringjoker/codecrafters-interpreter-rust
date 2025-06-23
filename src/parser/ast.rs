use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Expression(Expression),
    Statement(Statement),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    NOP, // Represents a no-operation expression
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(ReturnStatement),
    Declaration(Declaration),
    Print(PrintStatement),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NumberFloat(f64),
    NumberInteger(i64),
    String(String),
    Boolean(bool),
    NIL,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    And,
    Or,
    Assignment,
    Call,
    Dot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
    Not,
    Plus,
    Group,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub then_branch: Box<Statement>,
    pub else_branch: Box<Option<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub initializer: Box<Option<Statement>>,
    pub condition: Option<Expression>,
    pub increment: Box<Option<Statement>>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<String>,
    body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub root: Option<ASTNode>,
    pub nodes: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintStatement {
    pub value: Expression,
}

impl Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        Ok(())
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Expression(expr) => write!(f, "{}", expr),
            ASTNode::Statement(stmt) => write!(f, "{}", stmt),
            ASTNode::EOF => write!(f, ""),
        }
    }
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::BinaryOperation(op) => write!(f, "{}", op),
            Expression::UnaryOperation(op) => write!(f, "{}", op),
            Expression::NOP => write!(f, "NOP"),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::If(stmt) => write!(f, "{}", stmt),
            Statement::While(stmt) => write!(f, "{}", stmt),
            Statement::For(stmt) => write!(f, "{}", stmt),
            Statement::Return(stmt) => write!(f, "{}", stmt),
            Statement::Declaration(stmt) => write!(f, "{}", stmt),
            Statement::Block(stmt) => write!(f, "{}", stmt),
            Statement::Print(stmt) => write!(f, "{}", stmt),
            Statement::Expression(exp) => write!(f, "{};", exp),
        }
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Variable(var) => write!(f, "{}", var),
            Declaration::Function(func) => write!(f, "{}", func),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::NumberFloat(val) => write!(f, "{:?}", val),
            Literal::NumberInteger(val) => write!(f, "{}", val),
            Literal::String(val) => write!(f, "{}", val),
            Literal::Boolean(val) => write!(f, "{}", val),
            Literal::NIL => write!(f, "nil"),
        }
    }
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

impl Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator, self.operand)
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {}\n", self.condition)?;
        write!(f, "{}", &self.then_branch)?;
        if let Some(else_branch) = &self.else_branch.as_ref() {
            write!(f, "else")?;
            write!(f, "{}", else_branch)
        } else {
            Ok(())
        }
    }
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {}\n", self.condition)?;
        write!(f, "{}", self.body)
    }
}

impl Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for ")?;
        if let Some(init) = &self.initializer.as_ref() {
            write!(f, "{}; ", init)?;
        }
        if let Some(cond) = &self.condition {
            write!(f, "{}; ", cond)?;
        }
        if let Some(inc) = self.increment.as_ref() {
            write!(f, "{} ", inc)?;
        }
        write!(f, "  {}", self.body)
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "return {};", value)
        } else {
            write!(f, "return;")
        }
    }
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "var {} = {};", self.name, value)
        } else {
            write!(f, "var {};", self.name)
        }
    }
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}(", self.name)?;
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ") {{\n")?;
        for node in &self.body.statements {
            writeln!(f, "  {}", node)?;
        }
        write!(f, "}}")
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for node in &self.statements {
            writeln!(f, "  {}", node)?;
        }
        write!(f, "}}")
    }
}

impl Display for PrintStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "print {}", self.value)
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulus => write!(f, "%"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Or => write!(f, "or"),
            BinaryOperator::Assignment => write!(f, "="),
            BinaryOperator::Call => write!(f, "call"),
            BinaryOperator::Dot => write!(f, "."),
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Group => write!(f, "group"),
        }
    }
}
