//
// Created by Mark on 2022/6/27.
//
#include <iostream>

#include "lib.h"

void pass_to_c(int32_t* ptr, size_t size) {
  std::cout << "C++: I received the pointer!!\n";
  std::cout << "C++: I changed the value at offset 2 to [size].\n";
  *(ptr + 2) = (int32_t)size;

  print_array(ptr, size);
  // drop the array.
  drop_array(ptr, size);
}