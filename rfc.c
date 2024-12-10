#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Type_s Type;

struct Type_s {
    char *str;
    int len;

    // auto generate function pointers
    // void (*test_mutable_method)(Type *self);
    // void (*test_const_method)(const Type *self);
    // void (*test_copy_method)(Type self);
    // void (*test_static_method)();
};

Type *NewType(int len) {
    Type *self = (Type *)calloc(1, sizeof(Type));
    if (self == NULL)
    {
        return NULL;
    }

    self->str = (char *)malloc(sizeof(char) * len);
    if (self->str == NULL)
    {
        free(self);
        return NULL;
    }

    self->len = len;
    return self;
}

void DropType(Type **self) {
    if (self && *self) {
        free((*self)->str);
        free(*self);
        *self = NULL;
    }
}

void test_mutable_method(Type *self) {
}

void test_const_method(const Type *self) {
}

void test_copy_method(Type self) {
}

void test_static_method(void) {
}

// auto generate to bind struct with functions
// void BindType(Type *self) {
//     if (self) {
//         self->test_mutable_method = test_mutable_method;
//         self->test_const_method = test_const_method;
//         self->test_copy_method = test_copy_method;
//         self->test_static_method = test_static_method;
//     }
// }

void call() {
    Type *t = NewType(100);
    // auto call after creation to bind struct with functions
    // if (t) {
    //     BindType(t);
    // }

    // t->test_mutable_method(t);
    // t->test_const_method((const Type *)t);
    // t->test_copy_method(*t);
    // t->test_static_method();

    // auto call when leave scope to free resources
    // if(t) {
    //     DropType(&t);
    // }
}

Type *move1() {
    return NewType(100);
    // auto call after creation to bind struct with functions
    // Type *t = NewType(100);
    // if (t) {
    //     BindType(t);
    // }
    // return t;
}

Type *move2() {
    Type *t = NewType(99);
    // auto call after creation to bind struct with functions
    // if (t) {
    //     BindType(t);
    // }

    // t->test_mutable_method(t);
    // t->test_const_method((const Type *)t);
    // t->test_copy_method(*t);
    // t->test_static_method();

    return t;
}


int main() {
    call();

    Type *p1 = move1();
    // auto call when leave scope to free resources
    // if(p1) {
    //     DropType(&p1);
    // }

    Type *p2 = move2();
    // auto call when leave scope to free resources
    // if(p2) {
    //     DropType(&p2);
    // }
    
    return 0;
}
