const o = {x:0,c:[{x:1,c:[{x:2,c:[{x:3},{x:4,c:[{x:5}]},{x:6}]},{x:7},{x:8}]},{x:9}]}

const traverse = g => {
  const dfs = (stack, head) => (head.c || []).concat(stack)
  
  const loop = (acc, stack) => {
    if (stack.length === 0) {
        return acc
    }

    const [head, ...tail] = stack
    return loop(`${acc}/${head.x}`, dfs(tail, head))
  }
  
  return loop('', [g])
}

console.log(traverse(o))
console.log(traverse(o) === '/0/1/2/3/4/5/6/7/8/9')
