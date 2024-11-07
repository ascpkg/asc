// main.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "test_header.h"
#include "test_structs.h"
#include "test_functions.h"
#include "test_io.h"
#include "test_utils.h"

// condition compile example
#ifdef DEBUG
static void debug_print(const char *msg)
{
}
#endif

// macro defines example
#define MAX_BUFFER_SIZE 1024
#define SQUARE(x) ((x) * (x))

// type cast example
void type_casting_example(void)
{
    int i = 10;
    float f = (float)i;
    void *ptr = &i;
    int *int_ptr = (int *)ptr;
}

// bit op example
void bitwise_operations(void)
{
    unsigned int flags = 0;
    flags |= (1 << 0);  // set bit
    flags &= ~(1 << 1); // clear bit
    flags ^= (1 << 2);  // switch bit
}

// file op example
void file_operations(void)
{
    FILE *fp = fopen("test.txt", "r");
    if (fp)
    {
        fclose(fp);
    }
}

// alloc memmory example
void memory_management(void)
{
    int *array = (int *)malloc(10 * sizeof(int));
    if (array)
    {
        free(array);
    }
}

// struct array example
void struct_array_example(void)
{
    struct Point points[10];
    for (int i = 0; i < 10; i++)
    {
        points[i].x = i;
        points[i].y = i * 2;
    }
}

// function pointer array example
typedef int (*operation_fn)(int, int);

static int add(int a, int b) { return a + b; }
static int subtract(int a, int b) { return a - b; }
static int multiply(int a, int b) { return a * b; }
static int divide(int a, int b) { return b != 0 ? a / b : 0; }

void function_pointer_array(void)
{
    operation_fn ops[] = {add, subtract, multiply, divide};
    int result = ops[0](10, 5);
}

// recurse example
int factorial(int n)
{
    if (n <= 1)
        return 1;
    return n * factorial(n - 1);
}

// linked list op example
void linked_list_operations(void)
{
    struct Node *head = NULL;
    struct Node *current = create_node(1);
    head = current;

    for (int i = 2; i <= 5; i++)
    {
        current->next = create_node(i);
        current = current->next;
    }

    // clean
    while (head)
    {
        struct Node *temp = head;
        head = head->next;
        free(temp);
    }
}

// string op example
void string_operations(void)
{
    char str[100] = "Hello";
    strcat(str, " World");
    size_t len = strlen(str);
    char *dup = strdup(str);
    if (dup)
    {
        free(dup);
    }
}

// dynamic args example
void variable_args_example(void)
{
    int sum1 = sum_variable_args(3, 1, 2, 3);
    int sum2 = sum_variable_args(5, 1, 2, 3, 4, 5);
}

// entry
int main(void)
{
    // test
    type_casting_example();
    bitwise_operations();
    file_operations();
    memory_management();
    struct_array_example();
    function_pointer_array();
    linked_list_operations();
    string_operations();
    variable_args_example();

    return 0;
}