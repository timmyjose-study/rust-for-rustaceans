#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

typedef struct {
  bool tiny; // 1
  uint32_t normal; // 1 + 4 + 3 = 8
  uint8_t small; // 8 + 1 = 9
  uint64_t u64; // 9 + 8 = 17 + 7 = 24
  uint16_t u16; // 24 + 2 + 6 = 32
} Foo;

int main(int argc, char *argv[])
{
  Foo foo;
  printf("size of foo = %lu bytes\n", sizeof(foo));
  return 0;
}