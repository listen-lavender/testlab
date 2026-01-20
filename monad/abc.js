// Continuation Monad
const Cont = (runCont) => ({
  runCont,
  chain: (f) => Cont((resolve) => runCont((a) => f(a).runCont(resolve))),
  map: (f) => Cont((resolve) => runCont((a) => resolve(f(a)))),
});

Cont.of = (value) => Cont((resolve) => resolve(value));

// callCC :: ((a -> Cont b) -> Cont a) -> Cont a
const callCC = (f) =>
  Cont((resolve) => {
    f((value) => Cont(() => resolve(value))).runCont(resolve);
  });

// 示例：提前退出
const example = callCC((escape) =>
  Cont.of(10)
    .map((x) => x + 1)
    .chain((y) => escape(y * 100)) // 调用 escape，直接返回 y * 100
    .map((z) => z + 1) // 这行不会执行
);

example.runCont(console.log); // 输出: 1100（而不是 12）

const findUser = (users, name) =>
  callCC((escape) =>
    users.reduce(
      (cont, user) =>
        cont.chain(() =>
          // user.name === name ? escape(user) : Cont.of(null)
          {
            console.log("=======", user.id)
            if (user.name === name) {
                return escape(user);
            } else {
                return Cont.of(null);
            }
          }
        ),
      Cont.of(null)
    )
  );

const users = [
  { name: "Alice", id: 1 },
  { name: "Bob", id: 2 },
  { name: "Bod", id: 3 },
  { name: "Box", id: 4 },
];

findUser(users, "Bob").runCont(console.log); // 输出: { name: "Bob", id: 2 }
findUser(users, "Charlie").runCont(console.log); // 输出: null

// const tryCatch = (tryFn, catchFn) =>
//   callCC((escape) =>
//     tryFn().chain((result) => Cont.of(result)).catch((err) => escape(catchFn(err)))
//   );

// const computation = tryCatch(
//   () => Cont.of(42 / 0).map((x) => x + 1),
//   (err) => `Error: ${err}`
// );

// computation.runCont(console.log); // 输出: "Error: Infinity"


// 使用call/cc实现类似try-catch的控制流
const tryDivide = (a, b) => callCC(exit => 
    b === 0 
        ? exit("除数不能为零") 
        : Cont.of(a / b)
)

const computation = tryDivide(10, 2)
    .chain(result => Cont.of(`结果是: ${result}`))
    .chain(console.log)

computation.runCont(console.log) // 输出: "结果是: 5"

const errorComputation = tryDivide(10, 0)
    .chain(result => Cont.of(`结果是: ${result}`))
    .chain(console.log)

errorComputation.runCont(console.log) // 输出: "除数不能为零"

