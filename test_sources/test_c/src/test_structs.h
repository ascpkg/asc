// test_structs.h
#ifndef TEST_STRUCTS_H
#define TEST_STRUCTS_H

// nested array struct
struct ArrayContainer
{
    int array1d[10];
    int array2d[5][5];
    char strings[10][50];
};

// nested complex struct
struct Employee
{
    char name[50];
    int id;
    struct
    {
        int day;
        int month;
        int year;
    } birth_date;
    union
    {
        float hourly_rate;
        int salary;
    } payment;
    struct
    {
        char street[100];
        char city[50];
        char country[50];
        int postal_code;
    } address;
};

// linked list node
struct ListNode
{
    void *data;
    struct ListNode *next;
    struct ListNode *prev;
};

// tree node
struct TreeNode
{
    int value;
    struct TreeNode *left;
    struct TreeNode *right;
    struct TreeNode *parent;
};

// function pointer array
typedef int (*operation)(int, int);
extern operation operations[4];

#endif // TEST_STRUCTS_H