import {puzzle1, puzzle2} from "../src/Day1Old";
import {readFile} from "../src/Utils";

describe('Day1Old', () => {
    describe('puzzle1', () => {
        it('Expectation scenario', () => {
            const result = puzzle1(['199',
                '200',
                '208',
                '210',
                '200',
                '207',
                '240',
                '269',
                '260',
                '263']);

            expect(result).toBe(7);
        });

        it('real data scenario', () => {
            const data = readFile('Day1Old.txt');
            const result = puzzle1(data);

            expect(result).toBe(1559);
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {
            const result = puzzle2(['199',
                '200',
                '208',
                '210',
                '200',
                '207',
                '240',
                '269',
                '260',
                '263']);

            expect(result).toBe(5);
        });

        it('real data scenario', () => {
            const data = readFile('Day1Old.txt');
            const result = puzzle2(data);

            expect(result).toBe(1600);
        });
    });
});