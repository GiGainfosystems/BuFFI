#pragma once

#include "serde.hpp"
#include "bincode.hpp"

namespace BUFFI_NAMESPACE {

    struct DateTimeHelper {
        /// milliseconds since 1.1.1970 00:00:00
        int64_t milliseconds_since_unix_epoch;

        friend bool operator==(const DateTimeHelper&, const DateTimeHelper&);
        std::vector<uint8_t> bincodeSerialize() const;
        static DateTimeHelper bincodeDeserialize(std::vector<uint8_t>);
    };

    struct RandomEnum {

        /// An empty case that is here to make the test simpler
        struct NoValue {
            friend bool operator==(const NoValue&, const NoValue&);
            std::vector<uint8_t> bincodeSerialize() const;
            static NoValue bincodeDeserialize(std::vector<uint8_t>);
        };

        /// A timestamp from chrono that we would like to use in the API
        struct TimeStamp {
            BUFFI_NAMESPACE::DateTimeHelper value;

            friend bool operator==(const TimeStamp&, const TimeStamp&);
            std::vector<uint8_t> bincodeSerialize() const;
            static TimeStamp bincodeDeserialize(std::vector<uint8_t>);
        };

        std::variant<NoValue, TimeStamp> value;

        friend bool operator==(const RandomEnum&, const RandomEnum&);
        std::vector<uint8_t> bincodeSerialize() const;
        static RandomEnum bincodeDeserialize(std::vector<uint8_t>);
    };

    struct CustomType;

    /// A custom type that needs to be available in C++ as well
    struct CustomType {
        /// Some content
        int64_t some_content;
        /// A cyclic reference that's a bit more complex
        std::optional<serde::value_ptr<BUFFI_NAMESPACE::CustomType>> itself;
        /// An enum that contains a remote type that we would like to use in the API
        BUFFI_NAMESPACE::RandomEnum random_enum;
        /// A struct field using a proxy type for (de)serialization
        BUFFI_NAMESPACE::DateTimeHelper proxy;
        /// Test a type overwrite
        std::string overwrite;
        /// using a nested type also works
        std::vector<std::string> overwrite_2;
        /// This field uses a custom serialization and deserialization logic
        /// via serde
        std::string custom;

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

    struct Result_void_SerializableError {

        struct Ok {
            std::tuple<std::tuple<>> value;

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

        friend bool operator==(const Result_void_SerializableError&, const Result_void_SerializableError&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Result_void_SerializableError bincodeDeserialize(std::vector<uint8_t>);
    };

    struct Point1_f64 {
        double x;

        friend bool operator==(const Point1_f64&, const Point1_f64&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Point1_f64 bincodeDeserialize(std::vector<uint8_t>);
    };

} // end of namespace BUFFI_NAMESPACE


namespace BUFFI_NAMESPACE {

    inline bool operator==(const CustomType &lhs, const CustomType &rhs) {
        if (!(lhs.some_content == rhs.some_content)) { return false; }
        if (!(lhs.itself == rhs.itself)) { return false; }
        if (!(lhs.random_enum == rhs.random_enum)) { return false; }
        if (!(lhs.proxy == rhs.proxy)) { return false; }
        if (!(lhs.overwrite == rhs.overwrite)) { return false; }
        if (!(lhs.overwrite_2 == rhs.overwrite_2)) { return false; }
        if (!(lhs.custom == rhs.custom)) { return false; }
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
    serde::Serializable<decltype(obj.random_enum)>::serialize(obj.random_enum, serializer);
    serde::Serializable<decltype(obj.proxy)>::serialize(obj.proxy, serializer);
    serde::Serializable<decltype(obj.overwrite)>::serialize(obj.overwrite, serializer);
    serde::Serializable<decltype(obj.overwrite_2)>::serialize(obj.overwrite_2, serializer);
    serde::Serializable<decltype(obj.custom)>::serialize(obj.custom, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::CustomType serde::Deserializable<BUFFI_NAMESPACE::CustomType>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::CustomType obj;
    obj.some_content = serde::Deserializable<decltype(obj.some_content)>::deserialize(deserializer);
    obj.itself = serde::Deserializable<decltype(obj.itself)>::deserialize(deserializer);
    obj.random_enum = serde::Deserializable<decltype(obj.random_enum)>::deserialize(deserializer);
    obj.proxy = serde::Deserializable<decltype(obj.proxy)>::deserialize(deserializer);
    obj.overwrite = serde::Deserializable<decltype(obj.overwrite)>::deserialize(deserializer);
    obj.overwrite_2 = serde::Deserializable<decltype(obj.overwrite_2)>::deserialize(deserializer);
    obj.custom = serde::Deserializable<decltype(obj.custom)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const DateTimeHelper &lhs, const DateTimeHelper &rhs) {
        if (!(lhs.milliseconds_since_unix_epoch == rhs.milliseconds_since_unix_epoch)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> DateTimeHelper::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<DateTimeHelper>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline DateTimeHelper DateTimeHelper::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<DateTimeHelper>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::DateTimeHelper>::serialize(const BUFFI_NAMESPACE::DateTimeHelper &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.milliseconds_since_unix_epoch)>::serialize(obj.milliseconds_since_unix_epoch, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::DateTimeHelper serde::Deserializable<BUFFI_NAMESPACE::DateTimeHelper>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::DateTimeHelper obj;
    obj.milliseconds_since_unix_epoch = serde::Deserializable<decltype(obj.milliseconds_since_unix_epoch)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Point1_f64 &lhs, const Point1_f64 &rhs) {
        if (!(lhs.x == rhs.x)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Point1_f64::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Point1_f64>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Point1_f64 Point1_f64::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Point1_f64>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Point1_f64>::serialize(const BUFFI_NAMESPACE::Point1_f64 &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.x)>::serialize(obj.x, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Point1_f64 serde::Deserializable<BUFFI_NAMESPACE::Point1_f64>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::Point1_f64 obj;
    obj.x = serde::Deserializable<decltype(obj.x)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const RandomEnum &lhs, const RandomEnum &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> RandomEnum::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<RandomEnum>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline RandomEnum RandomEnum::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<RandomEnum>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::RandomEnum>::serialize(const BUFFI_NAMESPACE::RandomEnum &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::RandomEnum serde::Deserializable<BUFFI_NAMESPACE::RandomEnum>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::RandomEnum obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const RandomEnum::NoValue &lhs, const RandomEnum::NoValue &rhs) {
        return true;
    }

    inline std::vector<uint8_t> RandomEnum::NoValue::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<RandomEnum::NoValue>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline RandomEnum::NoValue RandomEnum::NoValue::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<RandomEnum::NoValue>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::RandomEnum::NoValue>::serialize(const BUFFI_NAMESPACE::RandomEnum::NoValue &obj, Serializer &serializer) {
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::RandomEnum::NoValue serde::Deserializable<BUFFI_NAMESPACE::RandomEnum::NoValue>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::RandomEnum::NoValue obj;
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const RandomEnum::TimeStamp &lhs, const RandomEnum::TimeStamp &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> RandomEnum::TimeStamp::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<RandomEnum::TimeStamp>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline RandomEnum::TimeStamp RandomEnum::TimeStamp::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<RandomEnum::TimeStamp>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::RandomEnum::TimeStamp>::serialize(const BUFFI_NAMESPACE::RandomEnum::TimeStamp &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::RandomEnum::TimeStamp serde::Deserializable<BUFFI_NAMESPACE::RandomEnum::TimeStamp>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::RandomEnum::TimeStamp obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
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

    inline bool operator==(const Result_void_SerializableError &lhs, const Result_void_SerializableError &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_void_SerializableError::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_void_SerializableError>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_void_SerializableError Result_void_SerializableError::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_void_SerializableError>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_void_SerializableError>::serialize(const BUFFI_NAMESPACE::Result_void_SerializableError &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_void_SerializableError serde::Deserializable<BUFFI_NAMESPACE::Result_void_SerializableError>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    BUFFI_NAMESPACE::Result_void_SerializableError obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_void_SerializableError::Ok &lhs, const Result_void_SerializableError::Ok &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_void_SerializableError::Ok::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_void_SerializableError::Ok>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_void_SerializableError::Ok Result_void_SerializableError::Ok::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_void_SerializableError::Ok>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_void_SerializableError::Ok>::serialize(const BUFFI_NAMESPACE::Result_void_SerializableError::Ok &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_void_SerializableError::Ok serde::Deserializable<BUFFI_NAMESPACE::Result_void_SerializableError::Ok>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_void_SerializableError::Ok obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace BUFFI_NAMESPACE {

    inline bool operator==(const Result_void_SerializableError::Err &lhs, const Result_void_SerializableError::Err &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Result_void_SerializableError::Err::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Result_void_SerializableError::Err>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Result_void_SerializableError::Err Result_void_SerializableError::Err::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Result_void_SerializableError::Err>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace BUFFI_NAMESPACE

template <>
template <typename Serializer>
void serde::Serializable<BUFFI_NAMESPACE::Result_void_SerializableError::Err>::serialize(const BUFFI_NAMESPACE::Result_void_SerializableError::Err &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
BUFFI_NAMESPACE::Result_void_SerializableError::Err serde::Deserializable<BUFFI_NAMESPACE::Result_void_SerializableError::Err>::deserialize(Deserializer &deserializer) {
    BUFFI_NAMESPACE::Result_void_SerializableError::Err obj;
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
