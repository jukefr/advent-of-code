import { readFileSync } from 'fs'
import { join } from 'path'

const digitMap = [
  ['one', '1'],
  ['two', '2'],
  ['three', '3'],
  ['four', '4'],
  ['five', '5'],
  ['six', '6'],
  ['seven', '7'],
  ['eight', '8'],
  ['nine', '9'],
  ['1', '1'],
  ['2', '2'],
  ['3', '3'],
  ['4', '4'],
  ['5', '5'],
  ['6', '6'],
  ['7', '7'],
  ['8', '8'],
  ['9', '9'],
]

// xtwone3four cases need to be handled

const response = readFileSync(join(__dirname, './input.txt'), 'utf8')
  .trim()
  .split('\n')
  .map(v => {
    const foundFirstValues = digitMap.map(digit => {
      return v.indexOf(digit[0])
    })
    let minimum = 99999999
    for (let index = 0; index < foundFirstValues.length; index++) {
      const element = foundFirstValues[index]
      if (element !== -1) {
        if (element < minimum) {
          minimum = element
        }
      }
    }
    const minimumIndex = foundFirstValues.indexOf(minimum)
    const firstDigit = digitMap[minimumIndex][1]

    const foundLastValues = digitMap.map(digit => {
      return v.lastIndexOf(digit[0])
    })
    let maximum = 0
    for (let index = 0; index < foundLastValues.length; index++) {
      const element = foundLastValues[index]
      if (element !== -1) {
        if (element > maximum) {
          maximum = element
        }
      }
    }
    const maximumIndex = foundLastValues.lastIndexOf(maximum)
    const lastDigit = digitMap[maximumIndex][1]

    console.log(v, firstDigit, lastDigit)
    return parseInt(`${firstDigit}${lastDigit}`)
  })
  // .map(v => v.match(/\d/g))
  // .map(v => parseInt(`${v[0]}${v[v.length - 1]}`))
  .reduce((prev, cur) => prev + cur, 0)

console.log(response)
