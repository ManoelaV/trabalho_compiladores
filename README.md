# Trabalhos — Linguagem Lugosi (JavaCC)

Este repositório contém os trabalhos da disciplina de **Projeto de Compiladores** para a
**Linguagem Lugosi**, implementados com **JavaCC**:

- **Trabalho 1** — analisador léxico (tokens da linguagem).
- **Trabalho 2** — analisador sintático (gramática completa), construído sobre o léxico
  do trabalho 1, com os mesmos tokens.

## Arquivos principais

- `Lugosi.jj` — especificação JavaCC do analisador **sintático** (e léxico) da Linguagem Lugosi: todos os tokens do trabalho 1 mais as produções da gramática (`Main`, `VarDecl`, `SeqComandos`, `Comando`, `Exp`, `Fator`, `Op`, `ListaExp`, `Funcion`, `ListaArg`).
- `teste.lug` — programa idêntico ao exemplo do enunciado do trabalho 1.
- `exemplo1.lug` — exemplo (funções, recursão, `bool`, `true`/`false`, `if`, `return`, `print`, operadores aritméticos e de comparação).
- `exemplo2.lug` — exemplo (`do { } while`, `while (...) do { }`, `&&`, `||`, `==`, `/`, literal com fração/expoente `1.5E2`, identificador com sublinhado `media_final`, chamada com lista de argumentos separada por vírgula).
- `programa1.lug` — **trabalho 2**: programa sintaticamente válido (funções com parâmetros, `var`, atribuição, `while ... do`, `if`, chamadas de função, `print`, `return`, expressões parentetizadas).
- `programa2.lug` — **trabalho 2**: programa sintaticamente válido (`do { } while`, blocos aninhados, `bool` com `true`/`false`, `&&`, `||`, `==`, `<`, `/`, expressão lógica aninhada, chamada com lista de argumentos).
- `sintaxe_erro.lug` — **trabalho 2**: programa sintaticamente **inválido** (falta o `;` após `x := 4`); o parser rejeita apontando o erro (linha 5, coluna 5, esperando `;`).
- `erro.lug` - exemplo de codigo com erro.
- `output_teste.txt`, `output_exemplo1.txt`, `output_exemplo2.txt` — saídas geradas pelo **analisador léxico (trabalho 1)** para os três programas acima.
- `esperado_teste.txt` — saída esperada conforme o enunciado do trabalho 1, usada para conferência (idêntica a `output_teste.txt`).
- `javacc.jar` — ferramenta JavaCC (v7.0.10).
- `Lugosi_professor_template.jj` — cópia do modelo mínimo enviado pelo professor, mantida apenas como referência (não entra na compilação).

## Pré-requisitos

- **JDK** 11+ — é preciso ter o `javac`. Testado com o **JDK 26** e com Temurin JDK 21.

> As classes deste repositório também foram compiladas com `javac --release 8`, por isso
> também executam em um JRE 8 (nesse caso os acentos podem aparecer "quebrados" no
> console do Windows, por causa do encoding do terminal, e não do programa).

## Como gerar, compilar e executar

Os comandos abaixo são os mesmos nos dois sistemas; a diferença é só a instalação do
JDK e a abertura do terminal. Rode sempre a partir da pasta do repositório.

### Windows (PowerShell ou Prompt de Comando)

```powershell
java -cp javacc.jar javacc Lugosi.jj
javac *.java
java Lugosi programa1.lug
java Lugosi programa2.lug
java Lugosi sintaxe_erro.lug
java Lugosi teste.lug
java Lugosi exemplo1.lug
java Lugosi exemplo2.lug
java Lugosi erro.lug
```

> No console do Windows os acentos das mensagens (ex.: `Análise sintática`) podem
> aparecer "quebrados" dependendo do *encoding* da terminal — é uma limitação do
> console, não do programa.

### Linux (Bash)

Se o JDK ainda não estiver instalado, primeiro instale-o (Debian/Ubuntu):

```bash
sudo apt update
sudo apt install default-jdk
```

Confirme com `java -version` e `javac -version`, depois no diretório do repositório:

```bash
git clone https://github.com/ManoelaV/trabalho_compiladores.git
cd trabalho_compiladores

java -cp javacc.jar javacc Lugosi.jj
javac *.java
java Lugosi programa1.lug
java Lugosi programa2.lug
java Lugosi sintaxe_erro.lug
```

- Os acentos das mensagens aparecem corretamente no terminal (o JDK usa UTF-8 por
  padrão a partir do Java 18).
- `programa1.lug` e `programa2.lug` devem terminar sem erros; `sintaxe_erro.lug` deve
  imprimir o erro sintático com linha e coluna e terminar com código de saída 1
  (confira com `echo $?`).

## Trabalho 2 — Analisador Sintático

O parser implementa a gramática da linguagem com a **recursão à esquerda eliminada**
(exigência do JavaCC, que não aceita recursão à esquerda nem alternativas iniciadas
pelo mesmo símbolo):

| Regra original (enunciado) | Forma implementada (iterativa) |
| --- | --- |
| `VARDECL -> VARDECL "var" TIPO id ";" \| vazio` | `( "var" TIPO id ";" )*` |
| `SEQCOMANDOS -> SEQCOMANDOS COMANDO \| vazio` | `COMANDO*` |
| `LISTAEXP -> EXP \| LISTAEXP "," EXP` | `EXP ( "," EXP )*` |
| `FUNC -> FUNC "function" ... \| "function" ...` | `( "function" ... )*` |
| `LISTAARG -> TIPO id \| LISTAARG "," TIPO id` | `TIPO id ( "," TIPO id )*` |

Observações:

- As duas alternativas de `COMANDO` que começam por `TOKEN_id` (atribuição e chamada)
  são distinguidas pelo segundo símbolo (`:=` ou `(`), resolvido pelo lookahead do JavaCC.
- `FATOR -> TOKEN_id ( "(" LISTAEXP? ")" )?` cobre tanto a variável quanto a chamada.
- As expressões seguem a gramática da linguagem: toda expressão binária precisa ser
  totalmente parentetizada (`( EXP OP EXP )`); `EXP` sem operador é um `FATOR`.
  Por isso, dentro de `if`/`while` (que já têm os próprios parênteses) a expressão
  aparece com um nível a mais, ex.: `while ((n > 1)) do { ... };`.
- O `main` do programa chama `Lugosi()` dentro de um `try`; se o programa estiver
  sintaticamente correto a execução termina com uma mensagem de sucesso; caso contrário,
  o `ParseException` é reportado no terminal com **linha e coluna** do erro e o processo
  termina com código de saída 1. Erros **léxicos** (`TokenMgrError`, ex.: caractere
  inválido como `@`) também são tratados e reportados com linha/coluna, sem stack trace.

### Resultados dos testes

| Programa | Resultado |
| --- | --- |
| `teste.lug`, `exemplo1.lug`, `exemplo2.lug` | sem erros sintáticos |
| `programa1.lug`, `programa2.lug` | sem erros sintáticos |
| `sintaxe_erro.lug` | `Erro sintático: Encountered "y" at line 5, column 5. Was expecting ";"` |
| `erro.lug` | `Erro léxico: Lexical error at line 4, column 11. Encountered: "@" (64)` |

## Tokens reconhecidos

Palavras reservadas:

`main`, `var`, `int`, `bool`, `if`, `while`, `do`, `return`, `print`, `function`, `true`, `false`

Operadores e símbolos:

`:=` (atribuição), `==` (igualdade), `&&`, `||`, `+`, `-`, `*`, `/`, `<`, `>`, `,`, `{`, `}`, `(`, `)`, `;`

Literais e identificadores:

- `NUM` — número literal: `digitos` `fracao_opcional` `expoente_opcional` (ex.: `9`, `10.5`, `1e-2`, `3.14E+10`).
- `ID` — identificador: `letra letraoudigito* finalsublinhado*` (ex.: `teste`, `a_1`, `media_final_2`).

## Saída esperada (Trabalho 1 — léxico)

Para o programa `teste.lug`:

```lugosi
main {
    var int teste;
    teste := 9;
    print(teste);
}
```

a saída é:

> No trabalho 2 o parser não imprime mais token a token; ele apenas valida a estrutura
> do programa e reporta erros sintáticos (com linha/coluna) quando existem. As mensagens
> abaixo são as do **trabalho 1** (analisador léxico).

```text
Palavra reservada: main
Abre Chaves: {
Palavra reservada: var
Palavra reservada: int
Identificador: teste
Ponto e vírgula: ;
Identificador: teste
Atribuição: :=
Número: 9
Ponto e vírgula: ;
Palavra reservada: print
Abre parênteses: (
Identificador: teste
Fecha parênteses: )
Ponto e vírgula: ;
Fecha Chaves: }
```

## Convenções léxicas adotadas

- O número literal **não** inclui sinal: em `-9.98e-10` o `-` é lexado como operador
  (`Subtração: -`) e `9.98e-10` como `Número`, coerente com a gramática, na qual `-`
  pertence a `OP`.
- O expoente aceita `e` minúsculo e `E` maiúsculo.
- `TOKEN_id` deve começar por letra; sublinhados só aparecem depois do primeiro
  caractere (`letraoudigito`), como em `a_1`, `media_final_2`.
- O texto das mensagens segue **exatamente** o formato do enunciado (`Abre Chaves`,
  `Fecha Chaves`, `Ponto e vírgula`, `Atribuição`, `Número`). A saída obtida para
  `teste.lug` é idêntica à saída esperada do PDF (veja `esperado_teste.txt`).
- Os acentos estão no `Lugosi.jj` como escapes Unicode (`\u00e1`, `\u00e7`, ...), o que
  garante a saída correta independentemente do *encoding* usado ao compilar.
- A linguagem **não possui comentários** na definição léxica do enunciado, então os
  arquivos `.lug` não podem conter `/* ... */` (o parser esperaria `main` na linha 1).
