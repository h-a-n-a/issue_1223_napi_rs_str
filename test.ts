import { transform, transformWithString } from "."

const res = transform({
  option1: {
    name: "whh"
  }
})
console.log(res)

// working as intended
const withString =  transformWithString({
  option1: {
    name: "whh"
  }
})
console.log(withString)