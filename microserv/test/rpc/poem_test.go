package poem

import (
    poemS "github.com/listen-lavender/testlab/microserv/rpc/server/poem"
    poemC "github.com/listen-lavender/testlab/microserv/rpc/client/poem"
    poemP "github.com/listen-lavender/testlab/microserv/proto/poem"
    context "golang.org/x/net/context"
    "google.golang.org/grpc"
    "net"
    "testing"
    "time"
    // "errors"
)

// func Division(a, b float64) (float64, error) {
//     if b == 0 {
//         return 0, errors.New("除数不能为0")
//     }
//     return a / b, nil
// }

func runPoemServer(addr string) *grpc.Server {
    service := poemS.NewPoemService()

    ln, err := net.Listen("tcp", addr)
    if err != nil {
        panic(err)
    }
    srv := poemS.MakeGRPCServer(service)
    s := grpc.NewServer()
    poemP.RegisterPoemServer(s, srv)

    go func() {
        s.Serve(ln)
    }()
    time.Sleep(time.Second)
    return s
}

func testAddAuthor(service poemP.PoemClient, ctx context.Context, author *poemP.Author){
    res, _ := service.AddAuthor(ctx, author)
    println(res.Stat.Code)
    println(res.Stat.Msg)
}

func testUpdateAuthor(service poemP.PoemClient, ctx context.Context, author *poemP.Author){
    res, _ := service.UpdateAuthor(ctx, author)
    println(res.Stat.Code)
    println(res.Stat.Msg)
}

func testDelAuthor(service poemP.PoemClient, ctx context.Context, author *poemP.Author){
    res, _ := service.DelAuthor(ctx, author)
    println(res.Stat.Code)
    println(res.Stat.Msg)
}

func testGetOneAuthor(service poemP.PoemClient, ctx context.Context, oneAuthorRequest *poemP.OneAuthorRequest){
    res, _ := service.GetOneAuthor(ctx, oneAuthorRequest)
    if res.Stat.Code == 0{
        println(res.Data.Id)
        println(res.Data.Name)
        println(res.Data.Age)
        println(res.Data.Sex)
    }else{
        println(res.Stat.Msg)
    }
}

func testAddPoem(service poemP.PoemClient, ctx context.Context, poem *poemP.PoemRecord){
    res, _ := service.AddPoem(ctx, poem)
    println(res.Stat.Code)
    println(res.Stat.Msg)
}

func testGetOnePoem(service poemP.PoemClient, ctx context.Context, onePoemRequest *poemP.OnePoemRequest){
    res, _ := service.GetOnePoem(ctx, onePoemRequest)
    if res.Stat.Code == 0{
        println(res.Data.Id)
        if res.Data.Author != nil{
            println(res.Data.Author.Name)
            println(res.Data.Author.Age)    
        }else{
            println("author not exists.")
        }
        println(res.Data.Title)
        println(res.Data.Content)    
    }else{
        println(res.Stat.Msg)
    }
    
}

func testUpdatePoem(service poemP.PoemClient, ctx context.Context, poem *poemP.PoemRecord){
    res, _ := service.UpdatePoem(ctx, poem)
    println(res.Stat.Code)
    println(res.Stat.Msg)
}

func testDelPoem(service poemP.PoemClient, ctx context.Context, poem *poemP.PoemRecord){
    res, _ := service.DelPoem(ctx, poem)
    println(res.Stat.Code)
    println(res.Stat.Msg)
}

func testGetPoems(service poemP.PoemClient, ctx context.Context, poemsRequest *poemP.PoemsRequest){
    res, _ := service.GetPoems(ctx, poemsRequest)
    if res.Stat.Code == 0{
        for index, poem := range(res.Data){
            println("----", index)
            println(poem.Id)
            if poem.Author != nil{
                println(poem.Author.Name)
                println(poem.Author.Age)
            }else{
                println("user not exists.")
            }
            println(poem.Title)
            println(poem.Content)
        }
    }else{
        println(res.Stat.Msg)
    }
}

func TestPoem(t *testing.T) {
    s := runPoemServer(":9999")
    defer s.GracefulStop()
    conn, err := grpc.Dial(":9999", grpc.WithInsecure(), grpc.WithTimeout(time.Second))
    if err != nil {
        panic(err)
    }
    defer conn.Close()
    service := poemC.NewPoemClient(conn)

    ctx := context.Background()
    var author *poemP.Author
    var oneAuthorRequest *poemP.OneAuthorRequest
    var poem *poemP.PoemRecord
    var onePoemRequest *poemP.OnePoemRequest
    var poemsRequest *poemP.PoemsRequest

    println("=======test add author")
    author = &poemP.Author{
        Id: 1,
        Name: "Jack",
        Age: 12,
        Sex: 1,
    }

    testAddAuthor(service, ctx, author)
    testAddAuthor(service, ctx, author)

    author = &poemP.Author{
        Id: 2,
        Name: "Suzan",
        Age: 13,
        Sex: 2,
    }
    testAddAuthor(service, ctx, author)

    author = &poemP.Author{
        Id: 3,
        Name: "David",
        Age: 14,
        Sex: 1,
    }
    testAddAuthor(service, ctx, author)

    println("=======test get one author")
    oneAuthorRequest = &poemP.OneAuthorRequest{
        Id: 2,
    }
    testGetOneAuthor(service, ctx, oneAuthorRequest)

    println("=======test update author")
    author = &poemP.Author{
        Id: 2,
        Name: "Lily",
        Age: 14,
    }
    testUpdateAuthor(service, ctx, author)

    oneAuthorRequest = &poemP.OneAuthorRequest{
        Id: 2,
    }
    testGetOneAuthor(service, ctx, oneAuthorRequest)

    println("=======test delete author")
    author = &poemP.Author{
        Id: 3,
    }
    testDelAuthor(service, ctx, author)

    oneAuthorRequest = &poemP.OneAuthorRequest{
        Id: 3,
    }
    testGetOneAuthor(service, ctx, oneAuthorRequest)

    println("=======test add poem")
    poem = &poemP.PoemRecord{
        Id:1,
        AuthorId:1,
        Title:"静夜思",
        Content:"床前明月光，疑是地上霜。\n举头望明月，低头思故乡。",
    }
    testAddPoem(service, ctx, poem)
    testAddPoem(service, ctx, poem)

    poem = &poemP.PoemRecord{
        Id:2,
        AuthorId:2,
        Title:"春晓",
        Content:"春眠不觉晓，处处闻啼鸟。\n夜来风雨声，花落知多少。",
    }
    testAddPoem(service, ctx, poem)

    poem = &poemP.PoemRecord{
        Id:3,
        AuthorId:3,
        Title:"草",
        Content:"离离原上草，一岁一枯荣。\n野火烧不尽，春风吹又生。",
    }
    testAddPoem(service, ctx, poem)

    println("=======test get one poem")
    onePoemRequest = &poemP.OnePoemRequest{
        Id:3,
    }
    testGetOnePoem(service, ctx, onePoemRequest)

    println("=======test update poem")
    poem = &poemP.PoemRecord{
        Id:3,
        Title:"野草",
    }
    testUpdatePoem(service, ctx, poem)
    onePoemRequest = &poemP.OnePoemRequest{
        Id:3,
    }
    testGetOnePoem(service, ctx, onePoemRequest)

    println("=======test delete poem")
    poem = &poemP.PoemRecord{
        Id:1,
    }
    testDelPoem(service, ctx, poem)
    onePoemRequest = &poemP.OnePoemRequest{
        Id:1,
    }
    testGetOnePoem(service, ctx, onePoemRequest)

    println("=======test get poems")
    poem = &poemP.PoemRecord{
        AuthorId:3,
        Title:"test",
        Content:"test",
    }
    poem.Id = 4
    testAddPoem(service, ctx, poem)
    poem.Id = 5
    testAddPoem(service, ctx, poem)
    poem.Id = 6
    testAddPoem(service, ctx, poem)
    poem.Id = 7
    testAddPoem(service, ctx, poem)
    poem.Id = 8
    testAddPoem(service, ctx, poem)
    poem.Id = 9
    testAddPoem(service, ctx, poem)
    poem.Id = 10
    testAddPoem(service, ctx, poem)
    poem.Id = 11
    testAddPoem(service, ctx, poem)
    poem.Id = 12
    testAddPoem(service, ctx, poem)

    poemsRequest = &poemP.PoemsRequest{
        Rangen: &poemP.Rangen{Page:1,Size:3},
    }
    testGetPoems(service, ctx, poemsRequest)

    poemsRequest = &poemP.PoemsRequest{
        Rangen: &poemP.Rangen{Page:2,Size:4},
    }
    testGetPoems(service, ctx, poemsRequest)

}

// func Test_Division_1(t *testing.T) {
//     if i, e := Division(6, 2); i != 3 || e != nil {
//         t.Error("Failed.")
//     } else {
//         t.Log("Succeed.")
//     }
// }

// func Test_Division_2(t *testing.T) {
//     t.Error("Failed.")
// }
