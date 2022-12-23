import {Stack} from "./Utils";

const PATTERN: RegExp = /^move (\d+) from (\d+) to (\d+)$/;

function initializeStacks(inputArray: string[], blankIndex: number): Stack<string>[] {
    let stacks: Stack<string>[] = [];
    let source = inputArray[blankIndex - 1];
    for (let index = 0; index < source.length; index++) {
        const stackHeader = source[index];
        if (!stackHeader || stackHeader.trim() == '') {
            continue;
        }

        const stackHeaderNum = +stackHeader;
        let currentStack = new Stack<string>();

        for (let stackLevelIndex = blankIndex - 2; stackLevelIndex >= 0; stackLevelIndex--) {
            let currentLevel = inputArray[stackLevelIndex];
            let currentValue = currentLevel[index];
            if (currentValue && currentValue.trim() != '') {
                currentStack.push(currentValue);
            }
        }

        stacks[stackHeaderNum] = currentStack;
    }


    return stacks;
}

const puzzle1 = (inputArray: string[]): string => {

    let blankIndex = inputArray.length;

    for (let index = 0; index < inputArray.length; index++) {
        if (inputArray[index].trim() == '') {
            blankIndex = index;
        }
    }


    const stacks: Stack<string>[] = initializeStacks(inputArray, blankIndex);
    for (let index = blankIndex + 1; index < inputArray.length; index++) {
        let inputString = inputArray[index];
        let tag = inputString.match(PATTERN);

        const quantity = +tag[1];
        const sourceStackIndex = +tag[2];
        const targetStackIndex = +tag[3];

        const sourceStack = stacks[sourceStackIndex];
        const targetStack = stacks[targetStackIndex];

        for (let quant = 0; quant < quantity; quant++) {
            let value = sourceStack.pop();
            targetStack.push(value);
        }

    }

    let result = '';

    for (const stack of stacks) {
        if (stack && stack.size() > 0) {
            result += stack.pop();
        }
    }


    return result;

}

const puzzle2 = (inputArray: string[]): string => {
    let blankIndex = inputArray.length;

    for (let index = 0; index < inputArray.length; index++) {
        if (inputArray[index].trim() == '') {
            blankIndex = index;
        }
    }


    const stacks: Stack<string>[] = initializeStacks(inputArray, blankIndex);
    for (let index = blankIndex + 1; index < inputArray.length; index++) {
        let inputString = inputArray[index];
        let tag = inputString.match(PATTERN);

        const quantity = +tag[1];
        const sourceStackIndex = +tag[2];
        const targetStackIndex = +tag[3];

        const sourceStack = stacks[sourceStackIndex];
        const targetStack = stacks[targetStackIndex];

        let tempStack = new Stack<string>();

        for (let quant = 0; quant < quantity; quant++) {
            let value = sourceStack.pop();
            tempStack.push(value);
        }

        for (let quant = 0; quant < quantity; quant++) {
            let value = tempStack.pop();
            targetStack.push(value);
        }

    }

    let result = '';

    for (const stack of stacks) {
        if (stack && stack.size() > 0) {
            result += stack.pop();
        }
    }


    return result;
}

export {puzzle1, puzzle2};

