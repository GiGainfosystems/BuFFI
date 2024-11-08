#pragma once

#include "serde.hpp"
#include "bincode.hpp"

namespace BUFFI_NAMESPACE {

    struct SerializableError {
        std::string message;

        friend bool operator==(const SerializableError&, const SerializableError&);
        std::vector<uint8_t> bincodeSerialize() const;
        static SerializableError bincodeDeserialize(std::vector<uint8_t>);
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
