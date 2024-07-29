function getUser(userId) {
  return function(ctx) {
    return ctx.users[userId];
  };
}

function setUser(userId, user) {
  return function(ctx) {
    ctx.users[userId] = user;
  };
}

function readerID(compute) {
  return ReaderMonad(function(ctx) {
    return function(callback) {
      callback(compute(ctx));
    }
  })
}

function ReaderMonad(reader) {
  reader.fmap = function(func){
    return ReaderMonad(function(ctx) {
      return function(callback) {
        reader(ctx)(function(v){
          callback(func(v))
        })
      }
    })
  }
  reader.applying = function(otherReader){
    return ReaderMonad(function(ctx) {
      return function(callback) {
        reader(ctx)(function(v){
          otherReader(v)(function(w){
            callback(w)
          })
        })
      }
    })
  }
  reader.bind = function(func){
    return ReaderMonad(function(ctx){
      return function(callback){
        reader(ctx)(function(v){
          func(v)(ctx)(callback)
        })
      }
    })
  }
  return reader
}

// function reader(k) {
//   return {
//     run: function(e) {
//       return k(e);
//     },
//     bind: function(f) {
//       return reader(function(e) {
//         return f(k(e)).run(e);
//       });
//     },
//     fmap: function(f) {
//       return reader(function(e) {
//         return f(k(e));
//       });
//     },
//   };
// }


function getUserName(userId) {
  return readerID(getUser(userId)).fmap(function(user) {
    return user.name;
  });
}

function setUserName(userId, newName) {
  return readerID(getUser(userId)).bind(function(user) {
    user.name = newName;
    return readerID(setUser(userId, user));
  });
}

function changeUserName(userId, newName) {
  return getUserName(userId).bind(function(oldName) {
    return setUserName(userId, newName).fmap(function() {
      return [ 'User'
             , userId
             , ': name changed from'
             , oldName
             , 'to'
             , newName
             ].join(' ');
    });
  });
}

var db = {
  users: {
    42: {
      id: 42,
      name: 'John Doe',
    },
    43: {
      id: 43,
      name: 'Mike Switch',
    }
  },
};

readerID(getUser(42)).fmap(user => user.name).fmap(x => x + " abc").applying(readerID(x => x + " def"))(db)(console.log);

function updateUser(oldUser) {
  return function(ctx) {
    newUser = {
      id: oldUser.id,
      name: oldUser.name + " Suzan ABC",
    };
    ctx.users[oldUser.id] = newUser;
    return "update success";
  };
}

// readerID(getUser(42)).bind(user => readerID(updateUser(user)))(db)(console.log);

// console.log(result)

// var result = changeUserName(42, 'Jack Doe')(db);

// console.log(result); // User 42 : name changed from John Doe to Jack Doe
// 2023-07-19
function gen() int {
    return 1
}

function ValueMonad(v) {
  v.fmap = function(func){
    return ValueMonad(function() {
        return func(v())
    })
  }
  v.applying = function(otherCont){
    return ValueMonad(function() {
        return otherCont()(v())
    })
  }
  v.bind = function(func){
    return ValueMonad(function() {
        return func(v())()
    })
  }
  return v
}

// 2024-07-07 神奇
function toCont(v) {
    return ReaderMonad(function() {
        return v;
    });
}

function ReaderMonad(cont) {
  cont.fmap = function(func){
    return ReaderMonad(function() {
        return func(cont());
    })
  }
  cont.applying = function(otherCont){
    return ReaderMonad(function() {
        return otherCont()(cont());
    })
  }
  cont.bind = function(func){
    return ReaderMonad(function() {
        return func(cont())();
    })
  }
  return cont
}

console.log(toCont(0).fmap(x => x + 1).fmap(x => x + 2).fmap(x => x + 3)());

console.log(toCont(0).applying(toCont(x => x + 1)).applying(toCont(x => x + 2)).applying(toCont(x => x + 3))());

console.log(toCont(0).bind(y => toCont(y + 1)).bind(y => toCont(y + 2)).bind(y => toCont(y + 3))());



