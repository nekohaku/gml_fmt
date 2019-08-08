use super::expressions::*;
use super::lex_token::Token;

pub type StmtBox<'a> = Box<StatementWrapper<'a>>;
pub type ParenInfo = (bool, bool);
pub type DeliminatedLines<'a> = Vec<DeliminatedLine<'a>>;

#[derive(Debug)]
pub struct StatementWrapper<'a> {
    pub statement: Statement<'a>,
    pub has_semicolon: bool,
}

impl<'a> StatementWrapper<'a> {
    pub fn new(statement: Statement<'a>, has_semicolon: bool) -> Box<StatementWrapper<'a>> {
        Box::new(StatementWrapper {
            statement,
            has_semicolon,
        })
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    VariableDeclList {
        var_decl: Vec<VariableDecl<'a>>,
    },
    EnumDeclaration {
        name: ExprBox<'a>,
        comments_after_lbrace: CommentsAndNewlines<'a>,
        members: DeliminatedLines<'a>,
    },
    ExpresssionStatement {
        expression: ExprBox<'a>,
    },
    Block {
        comments_after_lbrace: CommentsAndNewlines<'a>,
        statements: Vec<StmtBox<'a>>,
    },
    If {
        has_surrounding_paren: ParenInfo,
        condition: ExprBox<'a>,
        then_branch: StmtBox<'a>,
        else_branch: Option<StmtBox<'a>>,
    },
    While {
        has_surrounding_paren: ParenInfo,
        condition: ExprBox<'a>,
        body: StmtBox<'a>,
    },
    DoUntil {
        body: StmtBox<'a>,
        has_surrounding_paren: ParenInfo,
        condition: ExprBox<'a>,
    },
    Repeat {
        has_surrounding_paren: ParenInfo,
        condition: ExprBox<'a>,
        body: StmtBox<'a>,
    },
    For {
        initializer: Option<StmtBox<'a>>,
        condition: Option<ExprBox<'a>>,
        increment: Option<ExprBox<'a>>,
        body: StmtBox<'a>,
    },
    Return {
        expression: Option<ExprBox<'a>>,
    },
    Break,
    Exit,
    Switch {
        condition: ExprBox<'a>,
        comments_after_lbrace: CommentsAndNewlines<'a>,
        cases: Vec<Case<'a>>,
    },
    Comment {
        comment: Token<'a>,
    },
    MultilineComment {
        multiline_comment: Token<'a>,
    },
    RegionBegin {
        multi_word_name: Vec<Token<'a>>,
    },
    RegionEnd {
        multi_word_name: Vec<Token<'a>>,
    },
    Macro {
        macro_body: Vec<Token<'a>>,
    },
    Define {
        script_name: ExprBox<'a>,
        body: Vec<StmtBox<'a>>,
    },
    EOF,
}

#[derive(Debug)]
pub struct Case<'a> {
    pub comments_after_case: CommentsAndNewlines<'a>,
    pub case_type: CaseType<'a>,
    pub comments_after_colon: CommentsAndNewlines<'a>,
    pub statements: Vec<StmtBox<'a>>,
}

#[derive(Debug)]
pub enum CaseType<'a> {
    Case(ExprBox<'a>),
    Default,
}

#[derive(Debug)]
pub struct VariableDecl<'a> {
    pub var_expr: ExprBox<'a>,
    pub say_var: bool,
    pub assignment: Option<(CommentsAndNewlines<'a>, ExprBox<'a>)>,
}

#[derive(Debug)]
pub struct DeliminatedLine<'a> {
    pub expr: ExprBox<'a>,
    pub trailing_comment: Option<CommentsAndNewlines<'a>>,
}
