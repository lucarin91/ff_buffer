#include "ff_ubuffer.hpp"

extern "C" {
void *ffubuffer_build(uint64_t size) {
  ffubuffer *buffer = new ffubuffer(size);
  return buffer->init() ? buffer : NULL;
}

void ffubuffer_destroy(void *buffer) { free(buffer); }

bool ffubuffer_push(void *buffer, void *element) {
  return ((ffubuffer *)buffer)->push(element);
}

void *ffubuffer_pop(void *buffer) {
  void *element;
  bool ret = ((ffubuffer *)buffer)->pop(&element);
  return ret ? element : NULL;
}
}
