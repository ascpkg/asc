#include "a.h"
#include "b.h"
#include "c.h"

#include "a.hpp"
#include "b.hpp"
#include "c.hpp"

#include <stdint.h>

#include <string>


#if defined(_WIN32)
    #include <Windows.h>
#elif defined(__linux__)
    #include <linux/kernel.h>
#elif defined(__FreeBSD__) || defined(__NetBSD__) || defined(__OpenBSD__)
    #include <sys/queue.h>
#elif defined(__APPLE__) && defined(__MACH__)
    #if TARGET_OS_MAC == 1
        #include <mach/mach.h>
    #endif
#endif


int main(int argc, char **argv)
{
    return 0;
}

