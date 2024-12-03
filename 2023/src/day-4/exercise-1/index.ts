import { readFileSync } from 'fs'
import { join } from 'path'

const input = readFileSync(join(__dirname, './input.txt'), 'utf8')
  .trim()
  .split('\n')
  .map(l => l.split(/: +/)[1])
  .map(l => l.split(/ +\| +/))
  .map(l => l.map(m => m.split(/ +/)))
  .map(l => {
    let counter = 0
    l[1].map(result => {
      if (l[0].includes(result)) {
        counter += 1
      }
    })
    return counter
  })
  .map(l => {
    let count = 0
    for (let index = 1; index <= l; index++) {
      if (index === 1) {
        count = 1
      } else {
        count *= 2
      }
    }
    return count
  })
  .reduce((prev, cur) => prev + cur, 0)

console.log(input)
