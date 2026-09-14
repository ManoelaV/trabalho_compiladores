# Trabalho 1 — Analisador Léxico da Linguagem Lugosi (JavaCC)

Este repositório contém o **Trabalho 1** da disciplina de **Projeto de Compiladores**:
um **analisador léxico** para a **Linguagem Lugosi**, implementado com **JavaCC**.

## Arquivos principais

- `Lugosi.jj` — especificação JavaCC do analisador léxico da Linguagem Lugosi (todos os tokens previstos na gramática).
- `teste.lug` — programa idêntico ao exemplo do enunciado.
- `exemplo1.lug` — exemplo (funções, recursão, `bool`, `true`/`false`, `if`, `return`, `print`, operadores aritméticos e de comparação).
- `exemplo2.lug` — exemplo (`do { } while`, `while (...) do { }`, `&&`, `||`, `==`, `/`, literal com fração/expoente `1.5E2`, identificador com sublinhado `media_final`, chamada com lista de argumentos separada por vírgula).
- `erro.lug` - exemplo de codigo com erro. 
- `output_teste.txt`, `output_exemplo1.txt`, `output_exemplo2.txt` — saídas geradas pelo analisador para os três programas acima.
- `esperado_teste.txt` — saída esperada conforme o enunciado, usada para conferência (idêntica a `output_teste.txt`).
- `javacc.jar` — ferramenta JavaCC (v7.0.10).
- `Lugosi_professor_template.jj` — cópia do modelo mínimo enviado pelo professor, mantida apenas como referência (não entra na compilação).

## Pré-requisitos

- **JDK** 11+ — é preciso ter o `javac`. Testado com o **JDK 26** e com Temurin JDK 21.

> As classes deste repositório também foram compiladas com `javac --release 8`, por isso
> também executam em um JRE 8 (nesse caso os acentos podem aparecer "quebrados" no
> console do Windows, por causa do encoding do terminal, e não do programa).

## Como gerar, compilar e executar

```bash
java -cp javacc.jar javacc Lugosi.jj
javac *.java
java Lugosi teste.lug
java Lugosi exemplo1.lug
java Lugosi exemplo2.lug
java Lugosi erro.lug
```

## Tokens reconhecidos

Palavras reservadas:

`main`, `var`, `int`, `bool`, `if`, `while`, `do`, `return`, `print`, `function`, `true`, `false`

Operadores e símbolos:

`:=` (atribuição), `==` (igualdade), `&&`, `||`, `+`, `-`, `*`, `/`, `<`, `>`, `,`, `{`, `}`, `(`, `)`, `;`

Literais e identificadores:

- `NUM` — número literal: `digitos` `fracao_opcional` `expoente_opcional` (ex.: `9`, `10.5`, `1e-2`, `3.14E+10`).
- `ID` — identificador: `letra letraoudigito* finalsublinhado*` (ex.: `teste`, `a_1`, `media_final_2`).

## Saída esperada

Para o programa `teste.lug`:

```lugosi
main {
    var int teste;
    teste := 9;
    print(teste);
}
```

a saída é:

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
