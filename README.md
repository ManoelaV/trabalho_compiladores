# Trabalho — Analisador Léxico e Sintático (MiniRust)

Este repositório contém uma implementação de um analisador léxico e um parser para a linguagem MiniRust, construídos com JavaCC.

Arquivos principais:
- `Lugosi_analise_lexica.jj` — gramática/lexer que reconhece tokens MiniRust e imprime cada token no formato solicitado.
- `Lugosi.jj` — parser (substituído por `LugosiParser` na geração) com regras sem recursão à esquerda e precedência de operadores.
- `ex_mr1.rs`, `ex_mr2.rs` — dois exemplos de programas MiniRust que cobrem a maioria das construções da linguagem.

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

Próximos passos sugeridos
- Ajustar mensagens do parser para imprimir uma árvore sintática se necessário.
- Adicionar mais exemplos e casos de teste automatizados.

Se quiser, eu posso commitar essas mudanças no repositório ou gerar uma árvore sintática textual pelo parser.
