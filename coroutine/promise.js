function MockPromise(cont) {
    ab = {}
    cont(function(v){
       ab.then = function(callback){
          requestAnimationFrame(function(){
             callback(v);
          });
       }
    })
    return ab;
}

ab = MockPromise(function(resolve){
    console.log(4);
    resolve(5);
    console.log(6);
});
ab.then(console.log);
