#pragma once

#include <cstddef>
#include <limits>
#include <cstdint>

struct TestClient;

extern "C" size_t buffi_client_function(TestClient* this_ptr, const std::uint8_t* input, size_t input_size, std::uint8_t** out_ptr);
extern "C" size_t buffi_free_standing_function(const std::uint8_t* input, size_t input_size, std::uint8_t** out_ptr);
extern "C" void buffi_free_byte_buffer(std::uint8_t* ptr, size_t size);