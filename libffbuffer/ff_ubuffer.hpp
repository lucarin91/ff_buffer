#pragma once

#include "ff/ubuffer.hpp"

using ffubuffer = ff::uSWSR_Ptr_Buffer;

extern "C" {
void *ffubuffer_build(uint64_t size);
void ffubuffer_destroy(void *buffer);
bool ffubuffer_push(void *buffer, void *element);
void *ffubuffer_pop(void *buffer);
}
