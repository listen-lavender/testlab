function getUser(userId) {
  return function(ctx) {
    return function(callback) {
        callback(ctx.users[userId], 'get user.')
    }
  };
}

function setUser(userId, user) {
  return function(ctx) {
    ctx.users[userId] = user;
  };
}

function stateID(compute) {
  return StateMonad(function(ctx) {
    return function(callback) {
      compute(ctx)(function(val, log){
        callback(val, log);
      })
    }
  })
}

function StateMonad(reader) {
  reader.fmap = function(func){
    return StateMonad(function(ctx) {
      return function(callback) {
        reader(ctx)(function(v1, log1){
          func(v1)(function(v2, log2){
            callback(v2, log1+log2);
          })
        })
      }
    })
  }
  reader.applying = function(otherReader){
    return StateMonad(function(ctx) {
      return function(callback) {
        reader(ctx)(function(v1, log1){
          otherReader(v1)(function(v2, log2){
            callback(v2, log1+log2)
          })
        })
      }
    })
  }
  reader.bind = function(func){
    return StateMonad(function(ctx){
      return function(callback){
        reader(ctx)(function(v1, log1){
          func(v1)(ctx)(function(v2, log2){
            callback(v2, log1+log2);
          })
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
  return stateID(getUser(userId)).fmap(function(user) {
    return user.name;
  });
}

function setUserName(userId, newName) {
  return stateID(getUser(userId)).bind(function(user) {
    user.name = newName;
    return stateID(setUser(userId, user));
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

function log(val, msg){
  console.log(val, msg)
}

// function debugCube(x) {
//     return function(callback) {
//         callback(cube(x), 'cube was called.')
//     }
// }

// stateID(getUser(42)).fmap(user => callback => callback(user.name, "get user name.")).fmap(x => callback => callback(x + " abc", "add abc."))(db)(log);
stateID(getUser(42)).fmap(user => callback => callback(user.name, "get user name.")).fmap(x => callback => callback(x + " abc", "add abc.")).applying(stateID(x => callback => callback(x + " def", "add def.")))(db)(log);

function updateUser(oldUser) {
  return function(ctx) {
    newUser = {
      id: oldUser.id,
      name: oldUser.name + " Suzan ABC",
    };
    ctx.users[oldUser.id] = newUser;
    return function(callback) {
      callback(newUser, "update success");
    }
  };
}

stateID(getUser(42)).bind(user => stateID(updateUser(user)))(db)(log);

// console.log(result)

// var result = changeUserName(42, 'Jack Doe')(db);

// console.log(result); // User 42 : name changed from John Doe to Jack Doe

