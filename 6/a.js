const input = `Time:        56     71     79     99
Distance:   334   1135   1350   2430`

const parseInput = input => {
    const races = input.replace(/\w+:/g, "")
        .replace(/ +/g, " ")
        .split("\n")
        .map(l => l.trim().split(" ").map(n => parseInt(n)))
    return races[0].map((t, i) => [t, races[1][i]])
}

const calcDistance = totalTime => heldTime =>
    heldTime * (totalTime - heldTime)

const getWins = ([time, distance]) => {
    const calc = calcDistance(time)
    let count = 0
    for (let i = 1; i < time; i++) {
        if (calc(i) > distance)
            count++
    }
    return count
}

const races = parseInput(input)
const distances = races.map(getWins).reduce((acc, v) => acc * v)
console.log(distances)

console.log(getWins([56717999, 334113513502430]))
