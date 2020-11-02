#ifndef _LEXER_H_
#define _LEXER_H_

#include "tokens.h"

typedef struct
{
    char *source;
    Token *tokens;
    int current;
} Lexer;

void scan(Lexer *lexer);
Lexer new (char *source);

#endif