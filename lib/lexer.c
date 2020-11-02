#include "lexer.h"
#include <stdlib.h>
#include <stdio.h>

static int length(char *string)
{
    int i = 0;
    while (string[i] != '\0')
    {
        i++;
    }
    return i;
}

Lexer new (char *source)
{
    Lexer lexer;
    lexer.source = malloc(sizeof(source) * sizeof(char));
    lexer.source = source;
    lexer.current = -1;
    lexer.tokens = malloc(sizeof(Token) * (length(source) + 1));
    return lexer;
}

static char advance(Lexer *lexer)
{
    lexer->current++;
    return lexer->source[lexer->current - 1];
}

void scan(Lexer *lexer)
{
    printf("Triggered scan\n");
    int i = 0;
    char curchar = 'A';
    printf("%s\n", advance(lexer));
    while (curchar = advance(lexer) != '\0')
    {
        printf("Loop\n");
        switch (curchar)
        {
        case '+':
            lexer->tokens[i] = PLUS;
            break;
        case '-':
            lexer->tokens[i] = MINUS;
            break;
        case '<':
            lexer->tokens[i] = RIGHT_CHEVRON;
            break;
        case '>':
            lexer->tokens[i] = LEFT_CHEVRON;
            break;
        case '[':
            lexer->tokens[i] = LEFT_BRACE;
            break;
        case ']':
            lexer->tokens[i] = RIGHT_BRACE;
            break;
        case '.':
            lexer->tokens[i] = DOT;
            break;
        case ',':
            lexer->tokens[i] = COMA;
            break;

        default:
            printf("ERROR ! Invalid character (`%c`) at position %d.", curchar, lexer->current);
            exit(-5);
        }
        i++;
    }
    i++;
    lexer->tokens[i] = END;
}