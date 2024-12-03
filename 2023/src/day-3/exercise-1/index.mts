// node --loader ts-node/esm src/day-3/exercise-1/index.mts

import { readFileSync } from 'fs'
import { join } from 'path'
import * as url from 'url'
const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

import execAll from 'execall'

const symbols = ['*', '#', '-', '+', '@', '&', '%', '=', '$', '/']

const input = readFileSync(join(__dirname, './input.txt'), 'utf8').trim()

const inputSplitByLines = input.split('\n')
const inputSplitByCharacter = inputSplitByLines.map(line => line.split(''))

let total = 0
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
          total += parseInt(match.match)
          console.log('tl', total)
          break
        }
        // top middle
        if (
          symbols.includes(inputSplitByCharacter[lineIndex - 1][elementIndex])
        ) {
          total += parseInt(match.match)
          console.log('tm', total)
          break
        }
        // top right
        if (
          symbols.includes(
            inputSplitByCharacter[lineIndex - 1][elementIndex + 1],
          )
        ) {
          total += parseInt(match.match)
          console.log('tr', total)
          break
        }
      }
      // middle left
      if (
        symbols.includes(inputSplitByCharacter[lineIndex][elementIndex - 1])
      ) {
        total += parseInt(match.match)
        console.log('ml', total)
        break
      }
      // middle right
      if (
        symbols.includes(inputSplitByCharacter[lineIndex][elementIndex + 1])
      ) {
        total += parseInt(match.match)
        console.log('mr', total)
        break
      }
      if (lineIndex < inputSplitByLines.length - 1) {
        // bottom left
        if (
          symbols.includes(
            inputSplitByCharacter[lineIndex + 1][elementIndex - 1],
          )
        ) {
          total += parseInt(match.match)
          console.log('bl', total)
          break
        }
        // bottom middle
        if (
          symbols.includes(inputSplitByCharacter[lineIndex + 1][elementIndex])
        ) {
          total += parseInt(match.match)
          console.log('bm', total)
          break
        }
        // bottom right
        if (
          symbols.includes(
            inputSplitByCharacter[lineIndex + 1][elementIndex + 1],
          )
        ) {
          total += parseInt(match.match)
          console.log('br', total)
          break
        }
      }
    }
  })
})

console.log(total)
