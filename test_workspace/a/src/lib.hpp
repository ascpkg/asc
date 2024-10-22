#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

int A_API test(int left, int right);

#ifdef __cplusplus
}
#endif

class A_API Test
{
public:
    int add(int left, int right);
};

