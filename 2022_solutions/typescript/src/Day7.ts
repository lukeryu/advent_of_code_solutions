const REGEX: RegExp = /^(\d+) (\w+\.?\w*)$/;

const TOTAL_DISK_SPACE: number = 70000000;
const REQUIRED_DISK_SPACE: number = 30000000;

const get_directories = (current_directory: string[]): string[] => {
    let directories = [];
    for (let index = current_directory.length - 1; index >= 1; index--) {
        let slice = current_directory.slice(index)
            .reduce((accum, item) => {
                return accum + '/' + item;
            });
        directories.push(slice);
    }


    return directories;
}

const get_directory_sizes = (input_array: string[]): Map<string, number> => {
    let current_directory: string[] = [];
    let hash_map = new Map<string, number>();

    for (let input_string of input_array) {
        if (input_string.trim().startsWith("dir")) {
            // console.log("#dir "+input_string);
            continue;
        } else if (input_string.trim().startsWith("$ cd")) {
            // console.log("#cd "+input_string);
            if ("$ cd .." == input_string) {
                current_directory.pop();
            } else {
                current_directory.push(input_string.substring(5));
            }
        } else if (REGEX.test(input_string)) {
            // console.log("#regex Match "+input_string);
            let tag = input_string.trim().match(REGEX);

            let file_size = +tag[1];

            let directories: string[] = get_directories(current_directory);

            for (let dir of directories) {
                let value = hash_map.get(dir);
                if (!value) {
                    value = 0;
                }
                value += file_size;
                hash_map.set(dir, value);
            }
        } else {
            // console.log("#regex Doesnt Match "+input_string);
        }
    }

    return hash_map;
}

const puzzle1 = (input_array: string[]): number => {
    const hash_map = get_directory_sizes(input_array);

    let sum = 0;

    for (let value of hash_map.values()) {
        if (value < 100000) {
            console.log(value)
            sum += value;
        }
    }

    return sum;
}


const puzzle2 = (input_array: string[]): number => {
    let hash_map = get_directory_sizes(input_array);

    let root_value = hash_map.get("/");
    let remaining_space = TOTAL_DISK_SPACE - root_value;

    let space_needed = REQUIRED_DISK_SPACE - remaining_space;

    let min_thing_value = 70000000;

    for (let value of hash_map.values()) {
        if (value > space_needed) {
            if (value < min_thing_value) {
                min_thing_value = value;
            }
        }
    }


    return min_thing_value;
}

export {puzzle1, puzzle2};

