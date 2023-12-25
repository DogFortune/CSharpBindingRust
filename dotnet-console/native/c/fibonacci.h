#include <stdint.h>

typedef void* Fibonacci_t;

/* Create fibonacci instance */
Fibonacci_t* fibo_new();
/* Get latest fibonacci value */
uint32_t fibo_next(Fibonacci_t* handle);

void fibo(int count);