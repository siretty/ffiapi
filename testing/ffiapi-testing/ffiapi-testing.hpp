#pragma once

#include <cstdlib>

#include <exception>
#include <functional>
#include <iomanip>
#include <iostream>
#include <source_location>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

namespace ffiapi_testing {

struct registry
{
    struct entry
    {
        std::string name;
        std::function<void()> test;
    };

    std::vector<entry> entries;

    template <typename T, typename U>
    auto add(T && t, U && u) -> registry &
    {
        entries.emplace_back(std::forward<T>(t), std::forward<U>(u));
        return *this;
    }

    template <typename T, typename U>
    auto add_and_return_index(T && t, U && u) -> std::size_t
    {
        const auto index = entries.size();
        entries.emplace_back(std::forward<T>(t), std::forward<U>(u));
        return index;
    }

    int run()
    {
        decltype(entries) entries_to_run;
        entries_to_run.swap(entries);

        for (const auto & [name, test] : entries_to_run)
        {
            std::cerr << "running test " << std::quoted(name) << '\n';
            try
            {
                test();
            }
            catch (const std::exception & exception)
            {
                std::cerr << "test " << std::quoted(name) << " failed with exception: " << exception.what() << '\n';
                return EXIT_FAILURE;
            }
            catch (...)
            {
                std::cerr << "test " << std::quoted(name) << " failed with unknown exception" << '\n';
                return EXIT_FAILURE;
            }
        }

        return EXIT_SUCCESS;
    }

    static auto get() -> registry &
    {
        static registry instance;
        return instance;
    }
};

class assertion_failure : public std::exception
{
    std::string _message;

public:
    assertion_failure() = default;

    static auto from_expression_string(const std::string_view expression, const std::string_view description,
                                       const std::source_location source_location) -> assertion_failure
    {
        assertion_failure result{};
        result._message.append(source_location.file_name());
        result._message.push_back(':');
        result._message.append(std::to_string(source_location.line()));
        result._message.push_back(' ');
        result._message.append(source_location.function_name());
        result._message.append(": ");
        result._message.append(description);
        result._message.append(" failed: ");
        result._message.append(expression);
        return result;
    }

    [[nodiscard]] auto what() const noexcept -> const char * override
    {
        return _message.c_str();
    }
};

template <typename T>
void assert_true(const std::string_view s, T && t,
                 const std::source_location source_location = std::source_location::current())
{
    if (!std::forward<T>(t))
    {
        throw assertion_failure::from_expression_string(s, "assert true", source_location);
    }
}

template <typename T>
void assert_false(std::string_view s, T && t, std::source_location source_location = std::source_location::current())
{
    if (std::forward<T>(t))
    {
        throw assertion_failure::from_expression_string(s, "assert false", source_location);
    }
}

} // namespace ffiapi_testing

#define FFIAPI_TESTING_TEST_COUNTER __COUNTER__

#define FFIAPI_TESTING_TEST_IDENTIFIER_DETAILS(a, b) a##b

#define FFIAPI_TESTING_TEST_IDENTIFIER(a, b) FFIAPI_TESTING_TEST_IDENTIFIER_DETAILS(a, b)

#if defined(__GNUC__) && !defined(__clang__)
#define FFIAPI_TESTING_ATTR_NOINLINE [[gnu::noinline]]
#endif

#if defined(_MSC_VER) && !defined(__clang__)
#define FFIAPI_TESTING_ATTR_NOINLINE [[msvc::noinline]]
#endif

#if !defined(__GNUC__) && !defined(_MSC_VER) && defined(__clang__)
#define FFIAPI_TESTING_ATTR_NOINLINE [[clang::noinline]]
#endif

#define FFIAPI_TESTING_ADD_TEST_DETAILS(id, desc)                                                                      \
    void FFIAPI_TESTING_TEST_IDENTIFIER(_test_function_, id)();                                                        \
    const auto FFIAPI_TESTING_TEST_IDENTIFIER(_test_registration_, id) =                                               \
        ::ffiapi_testing::registry::get().add_and_return_index(desc,                                                   \
                                                               FFIAPI_TESTING_TEST_IDENTIFIER(_test_function_, id));   \
    FFIAPI_TESTING_ATTR_NOINLINE void FFIAPI_TESTING_TEST_IDENTIFIER(_test_function_, id)()

#define ASSERT_TRUE(...) ::ffiapi_testing::assert_true(#__VA_ARGS__, (__VA_ARGS__))

#define ASSERT_FALSE(...) ::ffiapi_testing::assert_false(#__VA_ARGS__, (__VA_ARGS__))

#define ADD_TEST(desc) FFIAPI_TESTING_ADD_TEST_DETAILS(FFIAPI_TESTING_TEST_COUNTER, desc)
