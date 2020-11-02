#include "parser.h"
#include <stdio.h>

static void runTok(Token tok, unsigned char *runtime)
{
    switch (tok)
    {
    case PLUS:
        ++(*runtime);
        break;
    case MINUS:
        --(*runtime);
        break;
    case LEFT_CHEVRON:
        runtime--;
        break;
    case RIGHT_CHEVRON:
        runtime++;
        break;
    case DOT:
        putchar(*runtime);
        break;
    case COMA:
        (*runtime) = getchar();
        break;

    default:
        break;
    }
}

static void loop(Lexer *lexer, int *i, unsigned char *runtime)
{
    while (lexer->tokens[*i] != LEFT_BRACE)
    {
        if (lexer->tokens[*i] == RIGHT_BRACE)
        {
            loop(lexer, i, runtime);
        }
        else
        {
            runTok(lexer->tokens[*i], runtime);
        }
        i++;
    }
}

void run(Lexer *lexer)
{
    printf("Triggered run");
    unsigned char runtime[1024];
    unsigned int i = 0;
    while (lexer->tokens[i] != END)
    {
        switch (lexer->tokens[i])
        {
        case RIGHT_BRACE:
            loop(lexer, &i, runtime);
            break;
        default:
            runTok(lexer->tokens[i], runtime);
            break;
        }
    }
}