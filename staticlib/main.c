#include <stdio.h>
#include <stdint.h>

struct Container {
    int32_t value;
};

// Defined in lib.rs
struct Container *new_boxed_container();
void free_boxed_container(struct Container *);

int main(int argc, char *argv[]) {
    struct Container *c = new_boxed_container();
    printf("value = %d\n", c->value);
    free_boxed_container(c);
    return 0;
}
