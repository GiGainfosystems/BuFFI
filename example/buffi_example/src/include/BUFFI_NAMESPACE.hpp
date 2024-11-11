#pragma once

#include "serde.hpp"
#include "bincode.hpp"

namespace BUFFI_NAMESPACE {

    struct CustomType;

    /// A custom type that needs to be available in C++ as well
    struct CustomType {
        /// Some content
        int64_t some_content;
        /// A cyclic reference that's a bit more complex
        std::optional<serde::value_ptr<BUFFI_NAMESPACE::CustomType>> itself;

        friend bool operator==(const CustomType&, const CustomType&);
        std::vector<uint8_t> bincodeSerialize() const;
        static CustomType bincodeDeserialize(std::vector<uint8_t>);
    };

    struct SerializableError {
        std::string message;

        friend bool operator==(const SerializableError&, const SerializableError&);
        std::vector<uint8_t> bincodeSerialize() const;
        static SerializableError bincodeDeserialize(std::vector<uint8_t>);
    };

    struct Result_CustomType_SerializableError {

        struct Ok {
            std::tuple<BUFFI_NAMESPACE::CustomType> value;

            friend bool operator==(const Ok&, const Ok&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Ok bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Err {
            std::tuple<BUFFI_NAMESPACE::SerializableError> value;

            friend bool operator==(const Err&, const Err&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Err bincodeDeserialize(std::vector<uint8_t>);
        };

        std::variant<Ok, Err> value;

        friend bool operator==(const Result_CustomType_SerializableError&, const Result_CustomType_SerializableError&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Result_CustomType_SerializableError bincodeDeserialize(std::vector<uint8_t>);
    };

    struct Result_String_SerializableError {

        struct Ok {
            std::tuple<std::string> value;

            friend bool operator==(const Ok&, const Ok&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Ok bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Err {
            std::tuple<BUFFI_NAMESPACE::SerializableError> value;

            friend bool operator==(const Err&, const Err&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Err bincodeDeserialize(std::vector<uint8_t>);
        };

        std::variant<Ok, Err> value;

        friend bool operator==(const Result_String_SerializableError&, const Result_String_SerializableError&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Result_String_SerializableError bincodeDeserialize(std::vector<uint8_t>);
    };

    struct Result_i64_SerializableError {

        struct Ok {
            std::tuple<int64_t> value;

            friend bool operator==(const Ok&, const Ok&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Ok bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Err {
            std::tuple<BUFFI_NAMESPACE::SerializableError> value;

            friend bool operator==(const Err&, const Err&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Err bincodeDeserialize(std::vector<uint8_t>);
        };

        std::variant<Ok, Err> value;

        friend bool operator==(const Result_i64_SerializableError&, const Result_i64_SerializableError&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Result_i64_SerializableError bincodeDeserialize(std::vector<uint8_t>);
    };

} // end of namespace BUFFI_NAMESPACE


namespace BUFFI_NAMESPACE {

    inline bool operator==(const CustomType &lhs, const CustomType &rhs) {
        if (!(lhs.some_content == rhs.some_content)) { return false; }
        if (!(lhs.itself == rhs.itself)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> CustomType::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<CustomType>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline CustomType CustomType::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<CustomType>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::CustomType>::serialize(const BUFFI_NAMESPACE::CustomType &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.some_content)>::serialize(obj.some_content, serializer);
    serde::Serializable<decltype(obj.itself)>::serialize(obj.itself, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::CustomType serde::Deserializable<BUFFI_NAMESPACE::CustomType>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::CustomType obj;
    obj.some_content = serde::Deserializable<decltype(obj.some_content)>::deserialize(deserializer);
    obj.itself = serde::Deserializable<decltype(obj.itself)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_CustomType_SerializableError &lhs, const Result_CustomType_SerializableError &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_CustomType_SerializableError::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_CustomType_SerializableError>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_CustomType_SerializableError Result_CustomType_SerializableError::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_CustomType_SerializableError>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_CustomType_SerializableError>::serialize(const BUFFI_NAMESPACE::Result_CustomType_SerializableError &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_CustomType_SerializableError serde::Deserializable<BUFFI_NAMESPACE::Result_CustomType_SerializableError>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::Result_CustomType_SerializableError obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_CustomType_SerializableError::Ok &lhs, const Result_CustomType_SerializableError::Ok &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_CustomType_SerializableError::Ok::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_CustomType_SerializableError::Ok>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_CustomType_SerializableError::Ok Result_CustomType_SerializableError::Ok::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_CustomType_SerializableError::Ok>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_CustomType_SerializableError::Ok>::serialize(const BUFFI_NAMESPACE::Result_CustomType_SerializableError::Ok &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_CustomType_SerializableError::Ok serde::Deserializable<BUFFI_NAMESPACE::Result_CustomType_SerializableError::Ok>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_CustomType_SerializableError::Ok obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_CustomType_SerializableError::Err &lhs, const Result_CustomType_SerializableError::Err &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_CustomType_SerializableError::Err::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_CustomType_SerializableError::Err>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_CustomType_SerializableError::Err Result_CustomType_SerializableError::Err::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_CustomType_SerializableError::Err>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_CustomType_SerializableError::Err>::serialize(const BUFFI_NAMESPACE::Result_CustomType_SerializableError::Err &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_CustomType_SerializableError::Err serde::Deserializable<BUFFI_NAMESPACE::Result_CustomType_SerializableError::Err>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_CustomType_SerializableError::Err obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_String_SerializableError &lhs, const Result_String_SerializableError &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_String_SerializableError::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_String_SerializableError>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_String_SerializableError Result_String_SerializableError::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_String_SerializableError>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_String_SerializableError>::serialize(const BUFFI_NAMESPACE::Result_String_SerializableError &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_String_SerializableError serde::Deserializable<BUFFI_NAMESPACE::Result_String_SerializableError>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::Result_String_SerializableError obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_String_SerializableError::Ok &lhs, const Result_String_SerializableError::Ok &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_String_SerializableError::Ok::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_String_SerializableError::Ok>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_String_SerializableError::Ok Result_String_SerializableError::Ok::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_String_SerializableError::Ok>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_String_SerializableError::Ok>::serialize(const BUFFI_NAMESPACE::Result_String_SerializableError::Ok &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_String_SerializableError::Ok serde::Deserializable<BUFFI_NAMESPACE::Result_String_SerializableError::Ok>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_String_SerializableError::Ok obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_String_SerializableError::Err &lhs, const Result_String_SerializableError::Err &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_String_SerializableError::Err::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_String_SerializableError::Err>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_String_SerializableError::Err Result_String_SerializableError::Err::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_String_SerializableError::Err>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_String_SerializableError::Err>::serialize(const BUFFI_NAMESPACE::Result_String_SerializableError::Err &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_String_SerializableError::Err serde::Deserializable<BUFFI_NAMESPACE::Result_String_SerializableError::Err>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_String_SerializableError::Err obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_i64_SerializableError &lhs, const Result_i64_SerializableError &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_i64_SerializableError::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_i64_SerializableError>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_i64_SerializableError Result_i64_SerializableError::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_i64_SerializableError>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_i64_SerializableError>::serialize(const BUFFI_NAMESPACE::Result_i64_SerializableError &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_i64_SerializableError serde::Deserializable<BUFFI_NAMESPACE::Result_i64_SerializableError>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::Result_i64_SerializableError obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_i64_SerializableError::Ok &lhs, const Result_i64_SerializableError::Ok &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_i64_SerializableError::Ok::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_i64_SerializableError::Ok>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_i64_SerializableError::Ok Result_i64_SerializableError::Ok::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_i64_SerializableError::Ok>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_i64_SerializableError::Ok>::serialize(const BUFFI_NAMESPACE::Result_i64_SerializableError::Ok &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_i64_SerializableError::Ok serde::Deserializable<BUFFI_NAMESPACE::Result_i64_SerializableError::Ok>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_i64_SerializableError::Ok obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_i64_SerializableError::Err &lhs, const Result_i64_SerializableError::Err &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_i64_SerializableError::Err::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_i64_SerializableError::Err>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_i64_SerializableError::Err Result_i64_SerializableError::Err::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_i64_SerializableError::Err>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_i64_SerializableError::Err>::serialize(const BUFFI_NAMESPACE::Result_i64_SerializableError::Err &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_i64_SerializableError::Err serde::Deserializable<BUFFI_NAMESPACE::Result_i64_SerializableError::Err>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_i64_SerializableError::Err obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const SerializableError &lhs, const SerializableError &rhs) {
        if (!(lhs.message == rhs.message)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> SerializableError::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<SerializableError>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline SerializableError SerializableError::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<SerializableError>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::SerializableError>::serialize(const BUFFI_NAMESPACE::SerializableError &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.message)>::serialize(obj.message, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::SerializableError serde::Deserializable<BUFFI_NAMESPACE::SerializableError>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::SerializableError obj;
    obj.message = serde::Deserializable<decltype(obj.message)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}
