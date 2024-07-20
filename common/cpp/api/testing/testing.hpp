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

namespace {

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

#define TEST_COUNTER __COUNTER__
#define TEST_IDENTIFIER_DETAILS(a, b) a##b
#define TEST_IDENTIFIER(a, b) TEST_IDENTIFIER_DETAILS(a, b)

#define ADD_TEST(...)                                                                                                  \
    const auto TEST_IDENTIFIER(_test_, TEST_COUNTER) = registry::get().add_and_return_index(__VA_ARGS__);

class assertion_failure : std::exception
{
    std::string _message;

public:
    assertion_failure() = default;

    static auto from_expression_string(std::string_view expression) -> assertion_failure
    {
        assertion_failure result{};
        result._message.append("assertion failed: ");
        result._message.append(expression);
        return result;
    }

    [[nodiscard]] auto what() const noexcept -> const char * override
    {
        return _message.c_str();
    }
};

template <typename T, typename U>
void assert_eq(std::string_view s, T && t, U && u,
               std::source_location source_location = std::source_location::current())
{
    if (!(std::forward<T>(t) == std::forward<U>(u)))
    {
        throw assertion_failure::from_expression_string(s);
    }
}

#define ASSERT_EQ(...) ::assert_eq(#__VA_ARGS__, __VA_ARGS__)

} // namespace