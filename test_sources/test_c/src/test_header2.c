#include "test_header.h"

// test_impl.c
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

// impl struct
struct Node *create_node(int data)
{
    return NULL;
}

// callback
void register_callback(callback_fn cb)
{
}

// dynamic args
int sum_variable_args(int count, ...)
{
    return 0;
}

// inline function
int my_max(int a, int b)
{
    return (a > b) ? a : b;
}
