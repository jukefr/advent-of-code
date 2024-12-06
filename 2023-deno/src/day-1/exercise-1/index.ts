import { readFileSync } from 'fs'
import { join } from 'path'

const response = readFileSync(join(__dirname, './input.txt'), 'utf8')
  .trim()
  .split('\n')
  .map(v => v.match(/\d/g))
  .map(v => parseInt(`${v[0]}${v[v.length - 1]}`))
  .reduce((prev, cur) => prev + cur, 0)

console.log(response)
