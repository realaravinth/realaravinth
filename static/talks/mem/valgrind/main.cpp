#include <stdio.h>
#include <stdlib.h>

#define BUF_SIZE 10

void seg_fault() {
    int n[10];
    int b[10];
    int rx[10];
    printf("%d\n", rx[5+234]);
}

void stack_buffer_overflow() {
    char foo[BUF_SIZE];
    char bar[BUF_SIZE];
//
    snprintf(foo, BUF_SIZE, "%s", "kasdlkfasdfasdfa dfaooo");
    printf("%s\n", bar);
}

void leak() { char* foo = (char*)malloc(sizeof(foo) * BUF_SIZE); 
free(foo);
}

void double_free() {
    char* foo = (char*)malloc(sizeof(foo) * BUF_SIZE);
    free(foo);
}

void invalid_write() {
    int* foo = (int*)malloc(sizeof(foo) * BUF_SIZE);
    foo[1] = 2234;
//    foo[BUF_SIZE * sizeof(foo) * 2222222] = 2234;
    free(foo);
}

void negative_malloc() {
 int* foo = (int*)malloc(1);
 free(foo);
}

int main() {
	int a = 234234234234234234234;
	printf("%d\n", a);
        seg_fault();
//        stack_buffer_overflow();
//        leak();
//        double_free();
//        invalid_write();
//		negative_malloc();
}
