import {reduceSum, toNumber} from "./Utils";

const getGassedAmount = (row: number, values: number[]): number => {
    return values.map((number) => {
        return Math.abs(row - number);
    }).reduce(reduceSum);
}

const getCostMap = (max: number): Map<number, number> => {
    const map = new Map<number, number>();

    let sum = 0;
    for (let index = 1; index <= max; index++) {
        sum += index;
        map.set(index, sum);
    }

    return map;
}

const getGassedSquaredAmount = (row: number, values: number[], costMap: Map<number, number>): number => {
    return values.map((number) => {
        return costMap.get(Math.abs(row - number));
    }).reduce(reduceSum);
}

const puzzle1 = (input: string): number => {
    const values = input.split(',');
    const numberValues: number[] = values.map(toNumber);

    const min = Math.min(...numberValues);
    const max = Math.max(...numberValues);


    let leastGassedPosition = min;
    let leastGassedAmount = Number.MAX_VALUE;
    for (let i = min + 1; i <= max; i++) {
        const currentGassedAmount = getGassedAmount(i, numberValues);
        if (currentGassedAmount < leastGassedAmount) {
            leastGassedAmount = currentGassedAmount;
            leastGassedPosition = i;
        }
    }
    return leastGassedAmount;
}

const puzzle2 = (input: string): number => {
    const values = input.split(',');
    const numberValues: number[] = values.map(toNumber);

    const min = Math.min(...numberValues);
    const max = Math.max(...numberValues);

    const costMap = getCostMap(max);

    let leastGassedPosition = min;
    let leastGassedAmount = Number.MAX_VALUE;
    for (let i = min + 1; i <= max; i++) {
        const currentGassedAmount = getGassedSquaredAmount(i, numberValues, costMap);
        if (currentGassedAmount < leastGassedAmount) {
            leastGassedAmount = currentGassedAmount;
            leastGassedPosition = i;
        }
    }
    return leastGassedAmount;
}

export {puzzle1, puzzle2};