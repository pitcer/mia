#include <assert.h>
#include <stdio.h>

static void print_array(int n, int* a) {
    for (size_t i = 0; i < n; i++) {
        printf("%d ", a[i]);
    }
}

inline static int count_odd(int n, int* a) {
    int count = 0;
    for (size_t i = 0; i < n; i++) {
        if (a[i] % 2 == 1) {
            count++;
        }
    }
    return count;
}

inline static int count_even(int n, int* a) {
    int count = 0;
    for (size_t i = 0; i < n; i++) {
        if (a[i] % 2 == 0) {
            count++;
        }
    }
    return count;
}

static inline int get_min(int a, int b) {
    return a <= b ? a : b;
}

static inline int min_even_sum(int elems, int n, int* a) {
    
}

int main(int argc, char const* argv[]) {
    int n;
    scanf("%d", &n);
    int a[n];
    for (size_t i = 0; i < n; i++) {
        scanf("%d", &a[i]);
    }

    const int even = count_even(n ,a);
    const int odd = count_odd(n, a);
    assert(even + odd == n);

    if (even - odd <= 1) {
        printf("0\n");
        return;
    }



    // if ||even| - |odd|| <= 1 => min = 0
    // wpp find |even| - |odd| - 1 min elements of the same parity
    // (first even parity, second odd parity and min will be an output)
    return 0;
}
