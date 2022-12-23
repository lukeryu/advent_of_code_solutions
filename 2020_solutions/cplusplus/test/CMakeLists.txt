enable_testing()

add_executable(advent_of_code_2020_test day1test.cpp day2test.cpp day5test.cpp day9test.cpp day12test.cpp day25test.cpp day8test.cpp day14test.cpp day24test.cpp day17test.cpp day16test.cpp day18test.cpp day23test.cpp algorithmtest.cpp day22test.cpp day19test.cpp utilstest.cpp)
find_package(GTest REQUIRED)
add_dependencies(advent_of_code_2020_test GTest::GTest)

target_link_libraries(advent_of_code_2020_test PRIVATE GTest::GTest advent_of_code_2020)

add_test(NAME advent_of_code_2020_test COMMAND advent_of_code_2020_test)

target_compile_options(advent_of_code_2020_test PRIVATE
        $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>
        $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -pedantic -Werror -fsanitize=address>
        )