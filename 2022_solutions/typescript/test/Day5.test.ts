import {puzzle1, puzzle2} from "../src/Day5";
import {readFile} from "../src/Utils";

describe('Day5', () => {

    const TEST_DATA: string[] = [
        '    [D]    ',
        '[N] [C]    ',
        '[Z] [M] [P]',
        ' 1   2   3 ',
        '',
        'move 1 from 2 to 1',
        'move 3 from 1 to 3',
        'move 2 from 2 to 1',
        'move 1 from 1 to 2'];

    describe('puzzle1', () => {
        it('Expectation scenario', () => {

            const result = puzzle1(TEST_DATA);

            expect(result).toBe('CMZ');
        });

        it('real data scenario', () => {
            const data = readFile('Day5.txt');
            const result = puzzle1(data);

            expect(result).toBe('TLFGBZHCN');
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {
            const result = puzzle2(TEST_DATA);

            expect(result).toBe('MCD');
        });

        it('real data scenario', () => {
            const data = readFile('Day5.txt');
            const result = puzzle2(data);

            expect(result).toBe('QRQFHFWCL');
        });
    });
});