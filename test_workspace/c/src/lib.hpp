#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

int C_API test(int left, int right);

#ifdef __cplusplus
}
#endif

class C_API Test
{
public:
    int add(int left, int right);
};

