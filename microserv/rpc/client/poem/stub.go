package poem

import (
    "io"
    "time"
    poemProto "github.com/listen-lavender/testlab/microserv/proto/poem"
    "github.com/listen-lavender/testlab/microserv/util"
    "github.com/go-kit/kit/endpoint"
    "github.com/go-kit/kit/sd"
    "github.com/go-kit/kit/sd/etcd"
    "github.com/go-kit/kit/sd/lb"
    grpctransport "github.com/go-kit/kit/transport/grpc"
    context "golang.org/x/net/context"
    "google.golang.org/grpc"
)

var poemClient poemProto.PoemClient

type PoemClient struct {
    AddAuthorEndpoint endpoint.Endpoint
    DelAuthorEndpoint endpoint.Endpoint
    UpdateAuthorEndpoint endpoint.Endpoint
    GetOneAuthorEndpoint endpoint.Endpoint
    AddPoemEndpoint endpoint.Endpoint
    DelPoemEndpoint endpoint.Endpoint
    UpdatePoemEndpoint endpoint.Endpoint
    GetOnePoemEndpoint endpoint.Endpoint
    GetPoemsEndpoint endpoint.Endpoint
}

func (p *PoemClient) AddAuthor(ctx context.Context, in *poemProto.Author, opts ...grpc.CallOption) (*poemProto.EmptyResponse, error){
    resp, err := p.AddAuthorEndpoint(ctx, in)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return resp.(*poemProto.EmptyResponse), nil
}
func (p *PoemClient) DelAuthor(ctx context.Context, in *poemProto.Author, opts ...grpc.CallOption) (*poemProto.EmptyResponse, error){
    resp, err := p.DelAuthorEndpoint(ctx, in)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return resp.(*poemProto.EmptyResponse), nil
}
func (p *PoemClient) UpdateAuthor(ctx context.Context, in *poemProto.Author, opts ...grpc.CallOption) (*poemProto.EmptyResponse, error){
    resp, err := p.UpdateAuthorEndpoint(ctx, in)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return resp.(*poemProto.EmptyResponse), nil
}
func (p *PoemClient) GetOneAuthor(ctx context.Context, in *poemProto.OneAuthorRequest, opts ...grpc.CallOption) (*poemProto.OneAuthorResponse, error){
    resp, err := p.GetOneAuthorEndpoint(ctx, in)
    if err != nil {
        return &poemProto.OneAuthorResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}, Data:nil}, nil
    }
    return resp.(*poemProto.OneAuthorResponse), nil
}
func (p *PoemClient) AddPoem(ctx context.Context, in *poemProto.PoemRecord, opts ...grpc.CallOption) (*poemProto.EmptyResponse, error){
    resp, err := p.AddPoemEndpoint(ctx, in)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return resp.(*poemProto.EmptyResponse), nil
}
func (p *PoemClient) DelPoem(ctx context.Context, in *poemProto.PoemRecord, opts ...grpc.CallOption) (*poemProto.EmptyResponse, error){
    resp, err := p.DelPoemEndpoint(ctx, in)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return resp.(*poemProto.EmptyResponse), nil
}
func (p *PoemClient) UpdatePoem(ctx context.Context, in *poemProto.PoemRecord, opts ...grpc.CallOption) (*poemProto.EmptyResponse, error){
    resp, err := p.UpdatePoemEndpoint(ctx, in)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return resp.(*poemProto.EmptyResponse), nil
}
func (p *PoemClient) GetOnePoem(ctx context.Context, in *poemProto.OnePoemRequest, opts ...grpc.CallOption) (*poemProto.OnePoemResponse, error){
    resp, err := p.GetOnePoemEndpoint(ctx, in)
    if err != nil {
        return &poemProto.OnePoemResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}, Data:nil}, nil
    }
    return resp.(*poemProto.OnePoemResponse), nil
}
func (p *PoemClient) GetPoems(ctx context.Context, in *poemProto.PoemsRequest, opts ...grpc.CallOption) (*poemProto.PoemsResponse, error){
    resp, err := p.GetPoemsEndpoint(ctx, in)
    if err != nil {
        return &poemProto.PoemsResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}, Data:nil}, nil
    }
    return resp.(*poemProto.PoemsResponse), nil
}

func Init(conn *grpc.ClientConn) {
    poemClient = NewPoemClient(conn)
}

func GetClient() poemProto.PoemClient {
    if poemClient == nil {
        panic("feed client is not be initialized!")
    }
    return poemClient
}

func NewPoemClient(conn *grpc.ClientConn) poemProto.PoemClient {
    var addAuthorEndpoint endpoint.Endpoint
    {
        addAuthorEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "AddAuthor",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.EmptyResponse{},
        ).Endpoint()
    }

    var delAuthorEndpoint endpoint.Endpoint
    {
        delAuthorEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "DelAuthor",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.EmptyResponse{},
        ).Endpoint()
    }

    var updateAuthorEndpoint endpoint.Endpoint
    {
        updateAuthorEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "UpdateAuthor",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.EmptyResponse{},
        ).Endpoint()
    }

    var getOneAuthorEndpoint endpoint.Endpoint
    {
        getOneAuthorEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "GetOneAuthor",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.OneAuthorResponse{},
        ).Endpoint()
    }

    var addPoemEndpoint endpoint.Endpoint
    {
        addPoemEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "AddPoem",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.EmptyResponse{},
        ).Endpoint()
    }

    var delPoemEndpoint endpoint.Endpoint
    {
        delPoemEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "DelPoem",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.EmptyResponse{},
        ).Endpoint()
    }

    var updatePoemEndpoint endpoint.Endpoint
    {
        updatePoemEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "UpdatePoem",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.EmptyResponse{},
        ).Endpoint()
    }

    var getOnePoemEndpoint endpoint.Endpoint
    {
        getOnePoemEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "GetOnePoem",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.OnePoemResponse{},
        ).Endpoint()
    }

    var getPoemsEndpoint endpoint.Endpoint
    {
        getPoemsEndpoint = grpctransport.NewClient(
            conn,
            "poem.Poem",
            "GetPoems",
            util.DummyEncode,
            util.DummyDecode,
            poemProto.PoemsResponse{},
        ).Endpoint()
    }

    return &PoemClient{
        AddAuthorEndpoint: addAuthorEndpoint,
        DelAuthorEndpoint: delAuthorEndpoint,
        UpdateAuthorEndpoint: updateAuthorEndpoint,
        GetOneAuthorEndpoint: getOneAuthorEndpoint,
        AddPoemEndpoint: addPoemEndpoint,
        DelPoemEndpoint: delPoemEndpoint,
        UpdatePoemEndpoint: updatePoemEndpoint,
        GetOnePoemEndpoint: getOnePoemEndpoint,
        GetPoemsEndpoint: getPoemsEndpoint,
    }
}

func MakeAddAuthorEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).AddAuthorEndpoint
}

func MakeDelAuthorEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).DelAuthorEndpoint
}

func MakeUpdateAuthorEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).UpdateAuthorEndpoint
}

func MakeGetOneAuthorEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).GetOneAuthorEndpoint
}

func MakeAddPoemEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).AddPoemEndpoint
}

func MakeDelPoemEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).DelPoemEndpoint
}

func MakeUpdatePoemEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).UpdatePoemEndpoint
}

func MakeGetOnePoemEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).GetOnePoemEndpoint
}

func MakeGetPoemsEndpoint(f poemProto.PoemClient) endpoint.Endpoint {
    return f.(*PoemClient).GetPoemsEndpoint
}

// Todo: use connect pool, and reference counting to one connection.
func PoemFactory(makeEndpoint func(f poemProto.PoemClient) endpoint.Endpoint) sd.Factory {
    return func(instance string) (endpoint.Endpoint, io.Closer, error) {
        conn, err := grpc.Dial(instance, grpc.WithInsecure())
        if err != nil {
            return nil, nil, err
        }
        service := NewPoemClient(conn)
        endpoint := makeEndpoint(service)
        return endpoint, conn, nil
    }
}


func InitWithSD(sdClient etcd.Client) {
    poemClient = NewPoemClientWithSD(sdClient)
}

func NewPoemClientWithSD(sdClient etcd.Client) poemProto.PoemClient {
    res := &PoemClient{}

    var factory sd.Factory
    var subscriber *etcd.Subscriber
    var balancer lb.Balancer
    var retry endpoint.Endpoint

    factory = PoemFactory(MakeAddAuthorEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.AddAuthorEndpoint = retry

    factory = PoemFactory(MakeDelAuthorEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.DelAuthorEndpoint = retry

    factory = PoemFactory(MakeUpdateAuthorEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.UpdateAuthorEndpoint = retry

    factory = PoemFactory(MakeGetOneAuthorEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.GetOneAuthorEndpoint = retry

    factory = PoemFactory(MakeAddPoemEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.AddPoemEndpoint = retry

    factory = PoemFactory(MakeDelPoemEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.DelPoemEndpoint = retry

    factory = PoemFactory(MakeUpdatePoemEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.UpdatePoemEndpoint = retry

    factory = PoemFactory(MakeGetOnePoemEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.GetOnePoemEndpoint = retry

    factory = PoemFactory(MakeGetPoemsEndpoint)
    subscriber, _ = etcd.NewSubscriber(sdClient, "/service/poem", factory, nil)
    balancer = lb.NewRoundRobin(subscriber)
    retry = lb.Retry(3, time.Second, balancer)
    res.GetPoemsEndpoint = retry

    return res
}

