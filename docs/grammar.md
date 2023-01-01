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
  | IF LPAREN <expr> RPAREN LBRACKET <statements> RBRACKET <else-if-statements>
  | IF LPAREN <expr> RPAREN LBRACKET <statements> RBRACKET <else-if-statements> ELSE LBRACKET <statements> RBRACKET
  | WHILE LPAREN <expr> RPAREN LBRACKET <statements> RBRACKET
  | <expr> SEMICOLON

else-if-statements :=
  | epsilon
  | ELSE IF <expr> RPAREN LBRACKET <statements> RBRACKET <rest-else-if-statements>

rest-else-if-statements :=
  | epsilon
  | <else-if-statement> <rest-else-if-statements>

expr :=
  | <logical_or_expr> <logical_or_expr'>

logical_or_expr :=
  | <logical_and_expr> <logical_or_expr'>

logical_or_expr' :=
  | epsilon
  | LOGIC_OR <logical_or_expr>

logical_and_expr :=
  | <equality_expr> <logical_and_expr'>

logical_and_expr' :=
  | epsilon
  | LOGIC_AND <logical_and_expr>

equality_expr :=
  | <relational_expr> <equality_expr'>

equality_expr' :=
  | epsilon
  | NE <equality_expr>
  | EQ <equality_expr>

relational_expr :=
  | <additive_expr> <relational_expr'>

relational_expr' :=
  | epsilon
  | LT <relational_expr>
  | GT <relational_expr>
  | LTE <relational_expr>
  | GTE <relational_expr>

additive_expr :=
  | <multiplicative_expr> <additive_expr'>

additive_expr' :=
  | epsilon
  | PLUS <additive_expr>
  | MINUS <additive_expr>
  | DIV <additive_expr>

multiplicative_expr :=
  | <unary_expr> <multiplicative_expr'>

multiplicative_expr' :=
  | epsilon
  | TIMES <multiplicative_expr>
  | MOD <multiplicative_expr>
  | DIV <multiplicative_expr>

unary_expr :=
  | <unary_expr'> <terminal>

unary_expr' :=
  | epsilon
  | LOGIC_NOT
  | MINUS

terminal :=
  | ID
  | ID LPAREN <args>
  | NUM
  | STRING
  | LPAREN <expr> RPAREN
  | LAM LPAREN <args> THIN_ARROW LBRACKET <statements> RBRACKET

args :=
  | RPAREN
  | <expr> <rest-args>

rest-args :=
  | RPAREN
  | COMMA <expr> <rest-args>
