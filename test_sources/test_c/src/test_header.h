// test_header.h
#ifndef TEST_HEADER_H
#define TEST_HEADER_H

// normal types defines
extern int global_integer;
extern char global_char;
extern float global_float;
extern double global_double;
extern long global_long;
extern short global_short;

// pointer defines
extern int *ptr_integer;
extern char *ptr_char;
extern void *ptr_void;

// array defines
extern int array_1d[];
extern int array_2d[][10];
extern char string_array[];

// normal function defines
int func_return_int(void);
void func_void_param(void);
int func_with_params(int a, char b, float c);
char *func_return_string(void);
int *func_return_ptr(void);

// function pointer defines
typedef int (*func_ptr_t)(int);
extern func_ptr_t func_ptr;

// simple struct
struct Point
{
    int x;
    int y;
};

// complex struct
struct Complex
{
    double real;
    double imag;
    struct Point point;
};

// nested struct
struct Node
{
    int data;
    struct Node *next;
};

// union type
union Data
{
    int i;
    float f;
    char str[20];
};

// enum type
enum Days
{
    SUNDAY,
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY
};

// struct pointer function
struct Node *create_node(int data);

// function callback
typedef void (*callback_fn)(int);
void register_callback(callback_fn cb);

// dynamic args
int sum_variable_args(int count, ...);

// bit
struct BitFields
{
    unsigned int bit1 : 1;
    unsigned int bit2 : 2;
    unsigned int bit3 : 3;
};

// const
extern const int CONSTANT_INT;
extern const char *CONSTANT_STRING;

// inline function
inline int my_max(int a, int b);

#endif // TEST_HEADER_H