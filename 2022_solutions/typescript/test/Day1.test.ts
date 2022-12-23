import {puzzle1, puzzle2} from "../src/Day1";
import {readFile} from "../src/Utils";

describe('Day1', () => {

    const testData = ['1000',
        '2000',
        '3000',
        '',
        '4000',
        '',
        '5000',
        '6000',
        '',
        '7000',
        '8000',
        '9000',
        '',
        '10000'];

    describe('puzzle1', () => {
        it('Expectation scenario', () => {

            const result = puzzle1(testData);

            expect(result).toBe(24000);
        });

        it('real data scenario', () => {
            const data = readFile('Day1.txt');
            const result = puzzle1(data);

            expect(result).toBe(71780);
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {
            const result = puzzle2(testData);

            expect(result).toBe(45000);
        });

        it('real data scenario', () => {
            const data = readFile('Day1.txt');
            const result = puzzle2(data);

            expect(result).toBe(212489);
        });
    });
});