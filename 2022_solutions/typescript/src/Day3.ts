const priority: { [key: string]: number } = {
    'a': 1,
    'b': 2,
    'c': 3,
    'd': 4,
    'e': 5,
    'f': 6,
    'g': 7,
    'h': 8,
    'i': 9,
    'j': 10,
    'k': 11,
    'l': 12,
    'm': 13,
    'n': 14,
    'o': 15,
    'p': 16,
    'q': 17,
    'r': 18,
    's': 19,
    't': 20,
    'u': 21,
    'v': 22,
    'w': 23,
    'x': 24,
    'y': 25,
    'z': 26,
    'A': 27,
    'B': 28,
    'C': 29,
    'D': 30,
    'E': 31,
    'F': 32,
    'G': 33,
    'H': 34,
    'I': 35,
    'J': 36,
    'K': 37,
    'L': 38,
    'M': 39,
    'N': 40,
    'O': 41,
    'P': 42,
    'Q': 43,
    'R': 44,
    'S': 45,
    'T': 46,
    'U': 47,
    'V': 48,
    'W': 49,
    'X': 50,
    'Y': 51,
    'Z': 52,
};

const getPriorityCost = (datum: String): number => {
    const halfLength = datum.length / 2;
    const secondCompartment = datum.substring(halfLength);

    for (let index = 0; index < halfLength; index++) {
        const datumElement = datum[index];
        if (secondCompartment.includes(datumElement)) {

            const priorityElement = priority[datumElement];
            return priorityElement;
        }
    }
}

const getBadgeConst = (data: string[], index: number): number => {
    const firstPack = data[index];
    const secondPack = data[index + 1];
    const thirdPack = data[index + 2];

    for (let packIndex = 0; packIndex < firstPack.length; packIndex++) {
        const datumElement = firstPack[packIndex];
        if (secondPack.includes(datumElement) && thirdPack.includes(datumElement)) {
            const priorityElement = priority[datumElement];
            return priorityElement;
        }
    }
}

const puzzle1 = (data: string[]): number => {
    let sum = 0;

    for (const datum of data) {
        sum += getPriorityCost(datum);
    }

    return sum;
}

const puzzle2 = (data: string[]): number => {
    let sum = 0;

    for (let index = 0; index < data.length; index = index + 3) {
        sum += getBadgeConst(data, index);
    }

    return sum;
}

export {puzzle1, puzzle2};

