#pragma once

#include <cstddef>
#include <limits>
#include "buffi_example_api_functions.hpp"

#include "BUFFI_NAMESPACE.hpp"


namespace BUFFI_NAMESPACE {

class TestClientHolder {
    TestClient* inner;
public:
    TestClientHolder(TestClient* ptr) {
        this->inner = ptr;
    }

    // An async function that needs a `Runtime` to be executed and returns a more complex type
    inline CustomType async_function(const int64_t& content) {
        auto serializer_content = serde::BincodeSerializer();
        serde::Serializable<int64_t>::serialize(content, serializer_content);
        std::vector<uint8_t> content_serialized = std::move(serializer_content).bytes();
        uint8_t* out_ptr = nullptr;

        size_t res_size = buffi_async_function(this->inner, content_serialized.data(), content_serialized.size(), &out_ptr);

        std::vector<uint8_t> serialized_result(out_ptr, out_ptr + res_size);
        Result_CustomType_SerializableError out = Result_CustomType_SerializableError::bincodeDeserialize(serialized_result);
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

    // A function that might use context provided by a TestClient to do its thing
    inline std::string client_function(const std::string& input) {
        auto serializer_input = serde::BincodeSerializer();
        serde::Serializable<std::string>::serialize(input, serializer_input);
        std::vector<uint8_t> input_serialized = std::move(serializer_input).bytes();
        uint8_t* out_ptr = nullptr;

        size_t res_size = buffi_client_function(this->inner, input_serialized.data(), input_serialized.size(), &out_ptr);

        std::vector<uint8_t> serialized_result(out_ptr, out_ptr + res_size);
        Result_String_SerializableError out = Result_String_SerializableError::bincodeDeserialize(serialized_result);
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

    // Here we use a type from a third party crate and return `()`
    inline void use_foreign_type_and_return_nothing(const Point1_f64& point) {
        auto serializer_point = serde::BincodeSerializer();
        serde::Serializable<Point1_f64>::serialize(point, serializer_point);
        std::vector<uint8_t> point_serialized = std::move(serializer_point).bytes();
        uint8_t* out_ptr = nullptr;

        size_t res_size = buffi_use_foreign_type_and_return_nothing(this->inner, point_serialized.data(), point_serialized.size(), &out_ptr);

        std::vector<uint8_t> serialized_result(out_ptr, out_ptr + res_size);
        Result_void_SerializableError out = Result_void_SerializableError::bincodeDeserialize(serialized_result);
        buffi_free_byte_buffer(out_ptr, res_size);

        if (out.value.index() == 0) { // Ok
            return;
        } else { // Err
            auto err = std::get<1>(out.value);
            auto error = std::get<0>(err.value);
            throw error;
        }
    }

};

}  // end of namespace BUFFI_NAMESPACE
