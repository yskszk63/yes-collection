Yes collection
--------------

Comparison of speeds of yes programs roughly written in each language.

## results

[here](https://yes-collection-web.vercel.app/)

## test code

bash
```bash
#!/bin/bash

while :; do echo y; done
```

c
```c
include <stdio.h>

int main(int argc, char* argv[]) {
    for (;;) {
        puts("y");
    }
}
```

deno
```javascript
while (true) {
    console.log("y");
}
```

go
```go
package main

import "fmt"

func main() {
	for ;; {
		fmt.Println("y")
	}
}
```

java
```java
public interface Yes {
    public static void main(String...args) {
        while (true) {
            System.out.println("y");
        }
    }
}
```

node
```javascript
!/usr/bin/env node

while (true) {
    console.log("y");
}
```

python
```python

#!/usr/bin/env python3

while True:
    print("y", flush=True)
```

rust
```
n main() {
    loop {
        println!("y")
    }
}
```
