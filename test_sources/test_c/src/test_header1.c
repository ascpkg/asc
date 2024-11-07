#include "test_header.h"

// test_impl.c
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

// global variable
int global_integer = 0;
char global_char = 0;
float global_float = 0.0f;
double global_double = 0.0;
long global_long = 0L;
short global_short = 0;

// pointers
int *ptr_integer = NULL;
char *ptr_char = NULL;
void *ptr_void = NULL;

// arrays
int array_1d[100];
int array_2d[10][10];
char string_array[100];

// const
const int CONSTANT_INT = 100;
const char *CONSTANT_STRING = "Hello";

// function pointer
func_ptr_t func_ptr = NULL;

// impl funtions
int func_return_int(void)
{
    return 0;
}

void func_void_param(void)
{
}

int func_with_params(int a, char b, float c)
{
    return 0;
}

char *func_return_string(void)
{
    return NULL;
}

int *func_return_ptr(void)
{
    return NULL;
}
