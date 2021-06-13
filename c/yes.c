#include <stdio.h>

int main(int argc, char* argv[]) {
    setvbuf(stdout, 0, _IONBF, 0); // fair?
    for (;;) {
        puts("y");
    }
}
