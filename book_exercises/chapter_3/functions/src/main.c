#include <stdio.h>

void raise_to_power(int x, int power) {
    size_t result = 1;

    for (int i = 1; i <= power; i++) {
        result *= x; 
    }
    printf("%d to the power of %d: %zu\n", x, power, result);
}

int main() {
    printf("Raise number to power.\n");

    int number, power;
    printf("Enter number: ");
    scanf("%d", &number);
    printf("\nEnter power: ");
    scanf("%d", &power);

    raise_to_power(number, power);

    return 0;
}
