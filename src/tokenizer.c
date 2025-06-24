#include "tokenizer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

token_t *init_token(char *value, int type) {
  token_t *token = calloc(1, sizeof(struct token_s));
  token->value = value;
  token->type = type;

  return token;
};

const char *token_type_to_str(int type) {
  switch (type) {
  case TOKEN_ID:
    return "TOKEN_ID";
  case TOKEN_EQUALS:
    return "TOKEN_EQUALS";
  case TOKEN_INT:
    return "TOKEN_INT";
  case TOKEN_SEMI:
    return "TOKEN_SEMI";
  case TOKEN_EOF:
    return "TOKEN_EOF";
  }

  return "not_stringable";
}

char *token_to_str(token_t *token) {
  const char *type_str = token_type_to_str(token->type);
  const char *template = "<type='%s', int_type='%d', value='%s'>";

  char *str = calloc(strlen(type_str) + strlen(template) + 8, sizeof(char));
  sprintf(str, template, type_str, token->type, token->value);

  return str;
}
