import { readFileSync } from 'fs'
import { join } from 'path'

const response = readFileSync(join(__dirname, './input.txt'), 'utf8')
  .trim()
  .split('\n')
  .map(v => v.split(': ')[1])
  .map(v => v.split('; '))
  .map(v => v.map(w => w.split(', ')))
  .map(v => {
    return v.map(w => {
      const tempArray = [0, 0, 0]
      w.map(x => {
        if (x.includes('red')) {
          tempArray[0] = parseInt(x.split(' ')[0])
        } else if (x.includes('green')) {
          tempArray[1] = parseInt(x.split(' ')[0])
        } else if (x.includes('blue')) {
          tempArray[2] = parseInt(x.split(' ')[0])
        }
      })
      return tempArray
    })
  })
  .map(v => {
    const maxes = [0, 0, 0]
    v.map(w => {
      if (w[0] > maxes[0]) maxes[0] = w[0]
      if (w[1] > maxes[1]) maxes[1] = w[1]
      if (w[2] > maxes[2]) maxes[2] = w[2]
    })
    return maxes
  })
  .map(v => v[0] * v[1] * v[2])
  .reduce((prev, cur) => prev + cur, 0)

console.log(response)
