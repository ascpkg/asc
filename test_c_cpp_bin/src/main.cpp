#include "wrapping.hpp"

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
