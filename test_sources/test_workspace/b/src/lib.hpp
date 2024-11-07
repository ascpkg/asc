#include "export.h"

#ifdef __cplusplus
extern "C" {
#endif

int B_API test(int left, int right);

#ifdef __cplusplus
}
#endif

class B_API Test
{
public:
    int add(int left, int right);
};

