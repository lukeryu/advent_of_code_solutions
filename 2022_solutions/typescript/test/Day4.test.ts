import {puzzle1, puzzle2} from "../src/Day4";
import {readFile} from "../src/Utils";

describe('Day4', () => {

    const TEST_DATA: string[] = ['2-4,6-8',
        '2-3,4-5',
        '5-7,7-9',
        '2-8,3-7',
        '6-6,4-6',
        '2-6,4-8'];

    describe('puzzle1', () => {
        it('Expectation scenario', () => {

            const result = puzzle1(TEST_DATA);

            expect(result).toBe(2);
        });

        it('real data scenario', () => {
            const data = readFile('Day4.txt');
            const result = puzzle1(data);

            expect(result).toBe(530);
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {
            const result = puzzle2(TEST_DATA);

            expect(result).toBe(4);
        });

        it('real data scenario', () => {
            const data = readFile('Day4.txt');
            const result = puzzle2(data);

            expect(result).toBe(903);
        });
    });
});