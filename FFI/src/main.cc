//
// Created by Mark on 2022/6/27.
//
#include <iostream>

#include "lib.h"

int main(int argc, const char *argv[]) {
  // Nullify arguments from main.
  (void)argc;
  (void)argv;

  std::cout << "C++: Calling Rust code...\n";
  some_function();

  auto* array = (int32_t*)(malloc(16));
  do_something_with_array(array, 16);

  std::cout << "C++: Rust modified the array!!\n";
  std::cout << array[0] << "\n";

  free(array);

  create_array_and_forget();

  return 0;
}
