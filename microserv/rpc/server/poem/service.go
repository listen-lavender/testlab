package poem

import (
    "errors"
    poemProto "github.com/listen-lavender/testlab/microserv/proto/poem"
    context "golang.org/x/net/context"
    "sync"
)

func Min(x, y int64) int64 {
    if x < y {
        return x
    }
    return y
}

func Max(x, y int64) int64 {
    if x > y {
        return x
    }
    return y
}


// Storage
var (
    mempoem map[int64]*poemProto.PoemRecord
    memuser map[int64]*poemProto.Author
    mu  sync.RWMutex
)

func init() {
    mempoem = make(map[int64]*poemProto.PoemRecord)
    memuser = make(map[int64]*poemProto.Author)
}

var (
    ErrUserNotFound = errors.New("user not found")
)

type service struct{}

// NewFeedService returns a naive, stateless implementation of Feed Service.
func NewPoemService() poemProto.PoemServer {
    return service{}
}

func (s service) AddAuthor(ctx context.Context, req *poemProto.Author) (*poemProto.EmptyResponse, error) {
    mu.Lock()
    defer mu.Unlock()
    _, ok := memuser[req.Id]
    if !ok {
        memuser[req.Id] = req
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:0, Msg:""}}, nil
    }
    return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:1, Msg:"Existed!"}}, nil
}

func (s service) DelAuthor(ctx context.Context, req *poemProto.Author) (*poemProto.EmptyResponse, error) {
    mu.Lock()
    defer mu.Unlock()
    _, ok := memuser[req.Id]
    if ok {
        delete(memuser, req.Id)
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:0, Msg:""}}, nil
    }
    return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:1, Msg:"Not existed!"}}, nil
}

func (s service) UpdateAuthor(ctx context.Context, req *poemProto.Author) (*poemProto.EmptyResponse, error) {
    mu.Lock()
    defer mu.Unlock()
    author, ok := memuser[req.Id]
    if ok {
        name := req.GetName()
        age := req.GetAge()
        sex := req.GetSex()
        flag := false
        if name != ""{
            author.Name = name
            flag = true
        }
        if age > 0{
            author.Age = age
            flag = true
        }
        if sex > 0{
            author.Sex = sex
            flag = true
        }
        if flag{
            return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:0, Msg:""}}, nil
        }else{
            return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:2, Msg:"No update"}}, nil
        }
        
    }
    return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:1, Msg:"Not existed!"}}, nil
}

func (s service) GetOneAuthor(ctx context.Context, req *poemProto.OneAuthorRequest) (*poemProto.OneAuthorResponse, error) {
    author, ok := memuser[req.Id]
    if ok {
        return &poemProto.OneAuthorResponse{Stat:&poemProto.Status{Code:0, Msg:""}, Data:author}, nil
    }
    return &poemProto.OneAuthorResponse{Stat:&poemProto.Status{Code:1, Msg:"Not existed!"}, Data:nil}, nil
}

func (s service) AddPoem(ctx context.Context, req *poemProto.PoemRecord) (*poemProto.EmptyResponse, error) {
    mu.Lock()
    defer mu.Unlock()
    _, ok := mempoem[req.Id]
    if !ok {
        mempoem[req.Id] = req
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:0, Msg:""}}, nil
    }
    return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:1, Msg:"Existed!"}}, nil
}

func (s service) DelPoem(ctx context.Context, req *poemProto.PoemRecord) (*poemProto.EmptyResponse, error){
    mu.Lock()
    defer mu.Unlock()
    _, ok := mempoem[req.Id]
    if ok {
        delete(mempoem, req.Id)
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:0, Msg:""}}, nil
    }
    return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:1, Msg:"Not existed!"}}, nil
}

func (s service) UpdatePoem(ctx context.Context, req *poemProto.PoemRecord) (*poemProto.EmptyResponse, error){
    mu.Lock()
    defer mu.Unlock()
    poem, ok := mempoem[req.Id]
    if ok {
        authorId := req.GetAuthorId()
        title := req.GetTitle()
        content := req.GetContent()
        flag := false
        if authorId > 0{
            poem.AuthorId = authorId
            flag = true
        }
        if title != ""{
            poem.Title = title
            poem.Content = content
            flag = true
        }
        if flag{
            return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:0, Msg:""}}, nil
        }else{
            return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:2, Msg:"No update"}}, nil
        }
        
    }
    return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:1, Msg:"Not existed!"}}, nil
}

func (s service) GetOnePoem(ctx context.Context, req *poemProto.OnePoemRequest) (*poemProto.OnePoemResponse, error) {
    poem, ok := mempoem[req.Id]
    if ok {
        author, _ := memuser[poem.AuthorId]
        data := &poemProto.PoemInfo{
            Id:poem.Id,
            Title:poem.Title,
            Content:poem.Content,
            Author:author,
        }
        return &poemProto.OnePoemResponse{Stat:&poemProto.Status{Code:0, Msg:""}, Data:data}, nil
    }
    return &poemProto.OnePoemResponse{Stat:&poemProto.Status{Code:1, Msg:"Not existed!"}, Data:nil}, nil
}

func (s service) GetPoems(ctx context.Context, req *poemProto.PoemsRequest) (*poemProto.PoemsResponse, error) {
    authorId := req.GetAuthorId()
    title := req.GetTitle()
    rangen := req.GetRangen()
    page := rangen.GetPage()
    size := rangen.GetSize()

    data := []*poemProto.PoemInfo{}
    for _, poem := range mempoem {
        if authorId != 0 && poem.AuthorId != authorId {
            continue
        }
        if title != "" && poem.Title != title {
            continue
        }
        author, _ := memuser[poem.AuthorId]
        poemInfo := &poemProto.PoemInfo{
            Id:poem.Id,
            Title:poem.Title,
            Content:poem.Content,
            Author:author,
        }
        data = append(data, poemInfo)
    }
    start := Max((page-1)*size, 0)
    end := Min(page*size, int64(len(data)))
    return &poemProto.PoemsResponse{Stat:&poemProto.Status{Code:0, Msg:""}, Data:data[start:end]}, nil
}

