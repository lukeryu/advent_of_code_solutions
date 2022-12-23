import {puzzle1, puzzle2} from "../src/Day2";
import {readFile} from "../src/Utils";

describe('Day2', () => {

    const testData = ['A Y',
        'B X',
        'C Z'];

    describe('puzzle1', () => {
        it('Expectation scenario', () => {
            const result = puzzle1(testData);

            expect(result).toBe(15);
        });

        it('real data scenario', () => {
            const data = readFile('Day2.txt');
            const result = puzzle1(data);

            expect(result).toBe(10941);
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {
            const result = puzzle2(testData);

            expect(result).toBe(12);
        });

        it('real data scenario', () => {
            const data = readFile('Day2.txt');
            const result = puzzle2(data);

            expect(result).toBe(13071);
        });
    });
});