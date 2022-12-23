import {start_of_message_marker, start_of_packet_marker} from "./Signals";

const puzzle1 = (string: string): number => {
    return start_of_packet_marker(string);
}

const puzzle2 = (string: string): number => {
    return start_of_message_marker(string);
}

export {puzzle1, puzzle2};

