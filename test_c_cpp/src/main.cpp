#include "config.h"
#include "version.h"
#include "wrapping.hpp"

#include <stdint.h>
#include <stdio.h>

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
    printf("%s\n", TEST_C_CPP_VERSION_STRING);

    #if defined(HAVE_GETTIMEOFDAY)
    printf("HAVE_GETTIMEOFDAY\n");
    #else
    printf("NOT HAVE_GETTIMEOFDAY\n");
    #endif

    fa();
    CA().a();
    CA().a(0);

    fb();
    CB().b();
    CB().b(0);

    fc(0);
    CC().c(0);
    CC().c(0, 0);

    return 0;
}
