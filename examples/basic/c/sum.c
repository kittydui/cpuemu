// expect rax = 55

int sum(int n);

// always put _start first.
// although this will be fixed,
// when we start loading programs at their preferred image base.
// however this is just being compiled to standalone code,
// not being loaded as a PE or ELF.
void _start(void) {
    int result = sum(10);
    __asm__ volatile("hlt" : : "a"(result));
}

int sum(int n) {
    int total = 0;
    for (int i = 1; i <= n; i++) {
        total += i;
    }
    return total;
}
