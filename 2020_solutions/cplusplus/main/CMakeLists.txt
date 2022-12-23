add_library(advent_of_code_2020 STATIC day1.cpp day2.cpp utils.cpp day5.cpp day9.cpp day12.cpp day25.cpp day8.cpp day14.cpp day24.cpp day17.cpp day16.cpp day18.cpp day23.cpp day22.cpp day19.cpp)

target_include_directories(advent_of_code_2020 PUBLIC includes)

target_compile_options(advent_of_code_2020 PRIVATE
        $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>
        $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -pedantic -Werror -fsanitize=address>
        )

find_package(Microsoft.GSL REQUIRED)
add_dependencies(advent_of_code_2020 Microsoft.GSL::GSL)

set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fno-omit-frame-pointer -fsanitize=address")
set(CMAKE_LINKER_FLAGS_DEBUG "${CMAKE_LINKER_FLAGS_DEBUG} -fno-omit-frame-pointer -fsanitize=address")