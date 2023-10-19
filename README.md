# yuio

A simple WIP toy language that compiles into x86_64 machine code with zero
dependencies.

## Overview

source code -> Lexer -> tokens\
tokens -> Parser -> ast\
ast -> ast::Node::generate() -> hlir\
hlir -> Lowerer -> llir\
llir -> x86_64::Compiler -> x86_64 instructions

## Example Script
[test_script.y](https://github.com/ohogb/yuio/blob/master/test_script.y)
