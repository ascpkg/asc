#pragma once

#include "export.h"

class TEST_PACKAGE_API CC
{
public:
    CC();
    virtual ~CC();

    bool c(int i);
    bool c(int i, int j);
};