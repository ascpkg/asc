#include "lib.hpp"
#include <iostream>
#include <fmt/format.h>

int test(int left, int right) {
    std::cout << fmt::format("{} + {} = {}\n", left, right, left + right);
    return left + right;
}


int Test::add(int left, int right) {
    return left + right;
}


/*
int main(int argc, char **argv) {
    int a = test(1, 2);
    int b = Test().add(1, 2);
}
*/
