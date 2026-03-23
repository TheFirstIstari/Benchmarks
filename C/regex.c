#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <regex.h>
#include <mach/mach_time.h>

#define ITERATIONS 1000
#define STR_LEN 100000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

int main(void) {
    printf("C Regex Benchmark (%d iterations)\n", ITERATIONS);
    
    char* test_str = malloc(STR_LEN + 1);
    for (int i = 0; i < STR_LEN; i++) {
        test_str[i] = (char)('a' + (i % 26));
    }
    test_str[STR_LEN] = '\0';
    
    regex_t re_find, re_count, re_email, re_complex;
    regcomp(&re_find, "xyz", REG_EXTENDED);
    regcomp(&re_count, "[aeiou]", REG_EXTENDED);
    regcomp(&re_email, "[a-z]+@[a-z]+\\.[a-z]+", REG_EXTENDED);
    regcomp(&re_complex, "[0-9]{3}-[0-9]{3}-[0-9]{4}", REG_EXTENDED);
    
    uint64_t t0, t1;
    double total_ms = 0;
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        regexec(&re_find, test_str, 0, NULL, 0);
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Find: %.2f ms (%.0f ops/sec)\n", 
           total_ms, ITERATIONS / (total_ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        regmatch_t pm;
        const char* pos = test_str;
        while (regexec(&re_count, pos, 1, &pm, 0) == 0) {
            pos += pm.rm_eo;
        }
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Count: %.2f ms (%.0f ops/sec)\n", 
           total_ms, ITERATIONS / (total_ms / 1000.0));
    
    char email_str[101];
    strncpy(email_str, test_str, 100);
    email_str[100] = '\0';
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        regexec(&re_email, email_str, 0, NULL, 0);
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Email match: %.2f ms (%.0f ops/sec)\n", 
           total_ms, ITERATIONS / (total_ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        regexec(&re_complex, test_str, 0, NULL, 0);
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Complex pattern: %.2f ms (%.0f ops/sec)\n", 
           total_ms, ITERATIONS / (total_ms / 1000.0));
    
    regfree(&re_find);
    regfree(&re_count);
    regfree(&re_email);
    regfree(&re_complex);
    free(test_str);
    
    return 0;
}
