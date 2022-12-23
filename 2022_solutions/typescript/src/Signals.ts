const count_unique_characters = (input_string: string): number => {
    const set = new Set<string>();

    for (let index = 0; index < input_string.length; index++) {
        set.add(input_string[index]);
    }

    return set.size;
}

const start_of_packet_marker = (input_string: string): number => {
    for (let index = 0; index < input_string.length - 3; index++) {
        const substring = input_string.substring(index, index + 4);
        const count = count_unique_characters(substring);

        if (count == 4) {
            return index + 4;
        }
    }

    return 0;
}

const start_of_message_marker = (input_string: string): number => {
    for (let index = 0; index < input_string.length - 13; index++) {
        const substring = input_string.substring(index, index + 14);
        const count = count_unique_characters(substring);

        if (count == 14) {
            return index + 14;
        }
    }

    return 0;
}

export {start_of_packet_marker, start_of_message_marker};