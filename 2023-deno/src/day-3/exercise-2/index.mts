// node --loader ts-node/esm src/day-3/exercise-2/index.mts

import { readFileSync } from 'fs'
import { join } from 'path'
import * as url from 'url'
const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

import execAll from 'execall'

const symbols = ['*']

const input = readFileSync(join(__dirname, './input.txt'), 'utf8').trim()

const inputSplitByLines = input.split('\n')
const inputSplitByCharacter = inputSplitByLines.map(line => line.split(''))

const starsArray: number[][] = []
inputSplitByLines.map((line, lineIndex) => {
  const matches = execAll(/(\d+)/g, line)

  matches.map(match => {
    for (let index = 0; index < match.match.length; index++) {
      const elementIndex = match.index + index

      // top left
      if (lineIndex > 0) {
        if (
          symbols.includes(
            inputSplitByCharacter[lineIndex - 1][elementIndex - 1],
          )
        ) {
          // star position line, star position index, number
          starsArray.push([
            lineIndex - 1,
            elementIndex - 1,
            parseInt(match.match),
          ])
          break
        }
        // top middle
        if (
          symbols.includes(inputSplitByCharacter[lineIndex - 1][elementIndex])
        ) {
          starsArray.push([lineIndex - 1, elementIndex, parseInt(match.match)])
          break
        }
        // top right
        if (
          symbols.includes(
            inputSplitByCharacter[lineIndex - 1][elementIndex + 1],
          )
        ) {
          starsArray.push([
            lineIndex - 1,
            elementIndex + 1,
            parseInt(match.match),
          ])
          break
        }
      }
      // middle left
      if (
        symbols.includes(inputSplitByCharacter[lineIndex][elementIndex - 1])
      ) {
        starsArray.push([lineIndex, elementIndex - 1, parseInt(match.match)])
        break
      }
      // middle right
      if (
        symbols.includes(inputSplitByCharacter[lineIndex][elementIndex + 1])
      ) {
        starsArray.push([lineIndex, elementIndex + 1, parseInt(match.match)])
        break
      }
      if (lineIndex < inputSplitByLines.length - 1) {
        // bottom left
        if (
          symbols.includes(
            inputSplitByCharacter[lineIndex + 1][elementIndex - 1],
          )
        ) {
          starsArray.push([
            lineIndex + 1,
            elementIndex - 1,
            parseInt(match.match),
          ])
          break
        }
        // bottom middle
        if (
          symbols.includes(inputSplitByCharacter[lineIndex + 1][elementIndex])
        ) {
          starsArray.push([lineIndex + 1, elementIndex, parseInt(match.match)])
          break
        }
        // bottom right
        if (
          symbols.includes(
            inputSplitByCharacter[lineIndex + 1][elementIndex + 1],
          )
        ) {
          starsArray.push([
            lineIndex + 1,
            elementIndex + 1,
            parseInt(match.match),
          ])
          break
        }
      }
    }
  })
})

const sortedStarsArray = starsArray.sort()

let total = 0
sortedStarsArray.map((star, starIndex) => {
  if (starIndex < sortedStarsArray.length - 2)
    if (
      `${star[0]}${star[1]}` ===
        `${sortedStarsArray[starIndex + 1][0]}${
          sortedStarsArray[starIndex + 1][1]
        }` &&
      `${star[0]}${star[1]}` !==
        `${sortedStarsArray[starIndex - 1][0]}${
          sortedStarsArray[starIndex - 1][1]
        }` &&
      `${star[0]}${star[1]}` !==
        `${sortedStarsArray[starIndex + 2][0]}${
          sortedStarsArray[starIndex + 2][1]
        }`
    ) {
      total += star[2] * sortedStarsArray[starIndex + 1][2]
    }
})

// need to handle firsts and lasts because i cant find a better conditionnal
// because i have a very tiny brain
// i mean... it works you know...
// probably breaks if firsts or lasts contain more than 2 identical idk
if (
  `${sortedStarsArray[0][0]}${sortedStarsArray[0][1]}` ===
    `${sortedStarsArray[1][0]}${sortedStarsArray[1][1]}` &&
  `${sortedStarsArray[0][0]}${sortedStarsArray[0][1]}` !==
    `${sortedStarsArray[2][0]}${sortedStarsArray[2][1]}`
) {
  total += sortedStarsArray[0][2] * sortedStarsArray[1][2]
}

if (
  `${sortedStarsArray[sortedStarsArray.length - 1][0]}${
    sortedStarsArray[sortedStarsArray.length - 1][1]
  }` ===
    `${sortedStarsArray[sortedStarsArray.length - 2][0]}${
      sortedStarsArray[sortedStarsArray.length - 2][1]
    }` &&
  `${sortedStarsArray[sortedStarsArray.length - 1][0]}${
    sortedStarsArray[sortedStarsArray.length - 1][1]
  }` !==
    `${sortedStarsArray[sortedStarsArray.length - 3][0]}${
      sortedStarsArray[sortedStarsArray.length - 3][1]
    }`
) {
  total +=
    sortedStarsArray[sortedStarsArray.length - 1][2] *
    sortedStarsArray[sortedStarsArray.length - 2][2]
}

console.log(total)
