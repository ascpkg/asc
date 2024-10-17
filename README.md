
```mermaid
flowchart LR;
    a/a.c ---> a/a.h;
    main.cpp ---> a/a.h;
    a/a.cpp ---> a/a.hpp;
    main.cpp ---> a/a.hpp;
    main.cpp ---> a/mod.hpp;
    b/b.c ---> b/b.h;
    main.cpp ---> b/b.h;
    b/b.cpp ---> b/b.hpp;
    main.cpp ---> b/b.hpp;
    main.cpp ---> b/mod.hpp;
    c/c.c ---> c/c.h;
    main.cpp ---> c/c.h;
    c/c.cpp ---> c/c.hpp;
    main.cpp ---> c/c.hpp;
    main.cpp ---> c/mod.hpp;
    main.cpp ---> wrapping.hpp;
```
