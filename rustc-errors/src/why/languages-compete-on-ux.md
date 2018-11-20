# Languages compete on UX

Example [from here](https://easyaspi314.github.io/gcc-vs-clang.html).

```c
#include <stdio.h>

int main(void) {
    printf("Hello, world!\n") // no semicolon
    return 0 // no semicolon
}
```

```text
~ $ gcc-7 -Wall semicolon.c 
semicolon.c: In function 'main':
semicolon.c:5:5: error: expected ';' before 'return'
     return 0 // no semicolon        
     ^~~~~~
```

```
C:\> cl /W3 /diagnostics:caret semicolon.c
semicolon.c(5,5): error C2143: syntax error: missing ';' before 'return'
    return 0 // no semicolon
    ^
semicolon.c(6,1): error C2143: syntax error: missing ';' before '}'
}
^
Microsoft (R) C/C++ Optimizing Compiler Version 19.10.25017 for x86
Copyright (C) Microsoft Corporation.  All rights reserved.
```

```
~ $ clang-6.0 -Wall semicolon.c
semicolon.c:4:30: error: expected ';' after expression
    printf("Hello, world!\n") // no semicolon
			     ^
			     ;
semicolon.c:5:13: error: expected ';' after return statement
    return 0 // no semicolon
	    ^
	    ;
2 errors generated.
```