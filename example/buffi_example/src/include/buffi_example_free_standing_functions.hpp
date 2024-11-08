#pragma once

#include <cstddef>
#include <limits>
#include "buffi_example_api_functions.hpp"

#include "BUFFI_NAMESPACE.hpp"


namespace BUFFI_NAMESPACE {

    // A function that is not part of an impl block
    inline int64_t free_standing_function(const int64_t& input) {
        auto serializer_input = serde::BincodeSerializer();
        serde::Serializable<int64_t>::serialize(input, serializer_input);
        std::vector<uint8_t> input_serialized = std::move(serializer_input).bytes();
        uint8_t* out_ptr = nullptr;

        size_t res_size = buffi_free_standing_function(input_serialized.data(), input_serialized.size(), &out_ptr);

        std::vector<uint8_t> serialized_result(out_ptr, out_ptr + res_size);
        Result_i64_SerializableError out = Result_i64_SerializableError::bincodeDeserialize(serialized_result);
        buffi_free_byte_buffer(out_ptr, res_size);

        if (out.value.index() == 0) { // Ok
            auto ok = std::get<0>(out.value);
            return std::get<0>(ok.value);
        } else { // Err
            auto err = std::get<1>(out.value);
            auto error = std::get<0>(err.value);
            throw error;
        }
    }


}  // end of namespace BUFFI_NAMESPACE
