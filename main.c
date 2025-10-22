// main.c
#include <some_lib.h>
#include <stdio.h>

int main(void)
{
    greet_from_rust();
    int sum = add_numbers(5, 7);
    printf("Sum from Rust: %d\n", sum);
    return 0;
}
