use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Expression(Expression),
    Statement(Statement),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Group(Box<Expression>),
    Literal(Literal),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(ReturnStatement),
    Declaration(Declaration),
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
    Group,
    Dot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
    Not,
    Plus,
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
    condition: Box<Expression>,
    then_branch: Vec<ASTNode>,
    else_branch: Option<Vec<ASTNode>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    condition: Box<Expression>,
    body: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    initializer: Option<Declaration>,
    condition: Option<Expression>,
    increment: Option<Expression>,
    body: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    name: String,
    value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<String>,
    body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    statements: Vec<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub root: Option<ASTNode>,
    pub nodes: Vec<ASTNode>,
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
            Expression::Group(node) => write!(f, "(group {})", node),
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
        write!(f, "if {} {{\n", self.condition)?;
        for node in &self.then_branch {
            writeln!(f, "  {}", node)?;
        }
        if let Some(else_branch) = &self.else_branch {
            writeln!(f, "}} else {{")?;
            for node in else_branch {
                writeln!(f, "  {}", node)?;
            }
        }
        write!(f, "}}")
    }
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {} {{\n", self.condition)?;
        for node in &self.body {
            writeln!(f, "  {}", node)?;
        }
        write!(f, "}}")
    }
}

impl Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for ")?;
        if let Some(init) = &self.initializer {
            write!(f, "{}; ", init)?;
        }
        if let Some(cond) = &self.condition {
            write!(f, "{}; ", cond)?;
        }
        if let Some(inc) = &self.increment {
            write!(f, "{} ", inc)?;
        }
        write!(f, "{{\n")?;
        for node in &self.body {
            writeln!(f, "  {}", node)?;
        }
        write!(f, "}}")
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
            BinaryOperator::Group => write!(f, "group"),
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
        }
    }
}
