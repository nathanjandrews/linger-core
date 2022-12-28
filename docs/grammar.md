# Linger Grammar

procedure :=
  | PROC ID LPAREN <params> LBRACKET <statements> RBRACKET

params :=
  | RPAREN
  | ID <rest-params>

rest-params :=
  | RPAREN
  | COMMA ID <rest-params>

statements :=
  | epsilon
  | statement <rest-statements>

rest-statements :=
  | <statement> <rest-statements>

statement :=
  | LET ID ASSIGN <expr> SEMICOLON
  | RETURN <expr> SEMICOLON
  | RETURN SEMICOLON
  | IF LPAREN <expr> RPAREN LBRACKET <statements> RBRACKET
  | IF LPAREN <expr> RPAREN LBRACKET <statements> RBRACKET ELSE LBRACKET <statements> RBRACKET
  | <expr> SEMICOLON

expr :=
  | <logical_or_expr> <logical_or_expr'>

logical_or_expr :=
  | <logical_and_expr> <logical_or_expr'>

logical_or_expr' :=
  | epsilon
  | LOGIC_OR <logical_or_expr>

logical_and_expr :=
  | <relational_expr> <logical_and_expr'>

logical_and_expr' :=
  | epsilon
  | LOGIC_AND <logical_and_expr>

relational_expr :=
  | <additive_expr> <relational_expr'>

relational_expr' :=
  | epsilon
  | EQ <relational_expr>
  | LT <relational_expr> // TODO: add LT and other relational operators

additive_expr :=
  | <terminal> <additive_expr'>

additive_expr' :=
  | epsilon
  | PLUS <additive_expr>
  | MINUS <additive_expr>

terminal :=
  | ID
  | ID LPAREN <args>
  | NUM
  | LPAREN <expr> RPAREN

args :=
  | RPAREN
  | <expr> <rest-args>

rest-args :=
  | RPAREN
  | COMMA <expr> <rest-args>
