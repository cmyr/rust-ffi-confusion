#include <stdint.h>
#include <stdio.h>

typedef struct HasVec {
    char **members;
    size_t len;
} HasVec;

extern HasVec* test_vec();

int main() {
printf("%d\n", 10);
	HasVec* vec = test_vec();
	for (int i = 0; i < vec->len; i++) {
		printf("%s\n", vec->members[i]);
	}
}
