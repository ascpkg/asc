#pragma once

#include "export.h"

class TEST_PACKAGE_API CD
{
public:
    CD();
    virtual ~CD();

    bool d(int i);
    bool d(int i, int j);
};