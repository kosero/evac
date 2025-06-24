#ifndef EVAC_TOKENIZER
#define EVAC_TOKENIZER

typedef struct token_s {
  char *value;
  enum {
    TOKEN_ID,
    TOKEN_EQUALS,
    TOKEN_INT,
    TOKEN_SEMI,
    TOKEN_EOF,
  } type;
} token_t;

token_t *init_token(char *value, int type);
const char *token_type_to_str(int type);
char *token_to_str(token_t *token);

#endif // !EVAC_TOKENIZER
