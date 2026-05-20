# Trabalho — MiniRust 1.0 com JavaCC

Este repositório contém a implementação de um analisador léxico e de um parser para a linguagem MiniRust 1.0, seguindo os enunciados fornecidos.

Arquivos principais:
- `Lugosi_analise_lexica.jj` — gramática léxica que reconhece os tokens previstos no enunciado e imprime cada token no formato solicitado.
- `Lugosi.jj` — gramática sintática do parser, sem recursão à esquerda e com suporte a main, funções, listas de argumentos, chamadas, if, while, return, println! e expressões com precedência.
- `ex_mr1.rs`, `ex_mr2.rs` — dois exemplos novos de programas MiniRust que exercitam boa parte das construções léxicas e sintáticas.

Requisitos:
- Java 11+ (javac/jre)
- JavaCC jar (fornecido como `javacc.jar` ou disponível para download)

Como gerar, compilar e executar

1) Baixar o JavaCC (caso não tenha `javacc.jar` no diretório):

```powershell
Invoke-WebRequest -Uri "https://repo1.maven.org/maven2/net/java/dev/javacc/javacc/7.0.10/javacc-7.0.10.jar" -OutFile javacc.jar -UseBasicParsing
```

2) Gerar o lexer e o parser a partir das gramáticas:

```powershell
java -cp javacc.jar javacc Lugosi_analise_lexica.jj
java -cp javacc.jar javacc Lugosi.jj
```

3) Compilar os arquivos Java gerados:

```powershell
javac *.java
```

4) Executar o analisador léxico (imprime tokens):

```powershell
java Lugosi ex_mr1.rs
java Lugosi ex_mr2.rs
```

5) Executar o parser (verifica sintaxe):

```powershell
java LugosiParser ex_mr1.rs
java LugosiParser ex_mr2.rs
```

Saída e testes
- Os arquivos `output_ex_mr1_lex.txt` e `output_ex_mr2_lex.txt` foram gerados durante testes e contêm a listagem de tokens para os exemplos.
- O parser imprime "Parse completo." quando a entrada é aceita pela gramática.
- A linguagem aceita segue a estrutura do enunciado: `fn main() { ... }` seguido de funções opcionais, `let`, atribuições, chamadas, `if`, `while`, `return`, `println!`, `read_float` e expressões com operadores.

Próximos passos sugeridos
- Ajustar mensagens do parser para imprimir uma árvore sintática se necessário.
- Adicionar mais exemplos e casos de teste automatizados.

Se quiser, eu posso commitar essas mudanças no repositório ou gerar uma árvore sintática textual pelo parser.
