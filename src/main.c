#include "tokenizer.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
  const char *scode = "let one = 1;";

  for (int i = 0; scode[i] != '\0'; i++) {
    char *val = (char[]){scode[i], '\0'};
    token_t *t = init_token(val, TOKEN_ID);
    char *t_str = token_to_str(t);

    printf("%s\n", t_str);

    free(t);
    free(t_str);
  }

  return 0;
}
