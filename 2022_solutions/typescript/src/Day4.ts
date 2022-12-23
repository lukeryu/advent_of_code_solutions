import {Range} from "./Utils";

const PATTERN: RegExp = /^([\d]+)-([\d]+),([\d]+)-([\d]+)$/;


const puzzle1 = (inputArray: string[]): number => {

    let count = 0;

    for (const inputString of inputArray) {
        let tag = inputString.match(PATTERN);

        let range1 = new Range(+tag[1], +tag[2]);
        let range2 = new Range(+tag[3], +tag[4]);

        if (range1.encompasses(range2) || range2.encompasses(range1)) {
            count++
        }

    }

    return count;
}

const puzzle2 = (inputArray: string[]): number => {
    let count = 0;

    for (const inputString of inputArray) {
        let tag = inputString.match(PATTERN);

        let range1 = new Range(+tag[1], +tag[2]);
        let range2 = new Range(+tag[3], +tag[4]);

        if (range1.overlaps(range2) || range2.overlaps(range1)) {
            count++
        }

    }

    return count;
}

export {puzzle1, puzzle2};

