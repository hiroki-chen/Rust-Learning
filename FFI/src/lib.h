//
// Created by Mark on 2022/6/27.
//
#include <cstddef>
#include <cstdint>

#pragma clang diagnostic push
#pragma ide diagnostic ignored "OCUnusedGlobalDeclarationInspection"

extern "C" {
void some_function(void);
void do_something_with_array(int32_t* ptr, size_t size);
void drop_array(int32_t* ptr, size_t size);
void create_array_and_forget();
void pass_to_c(int32_t* ptr, size_t size);
void print_array(const int32_t* array, size_t size);
}

#pragma clang diagnostic pop