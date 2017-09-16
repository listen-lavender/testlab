package poem

import (
    poemProto "github.com/listen-lavender/testlab/microserv/proto/poem"
    "github.com/go-kit/kit/endpoint"
    grpctransport "github.com/go-kit/kit/transport/grpc"
    context "golang.org/x/net/context"
)

type grpcServer struct {
    addauthor grpctransport.Handler
    delauthor grpctransport.Handler
    updateauthor grpctransport.Handler
    getoneauthor grpctransport.Handler
    addpoem grpctransport.Handler
    delpoem grpctransport.Handler
    updatepoem grpctransport.Handler
    getonepoem grpctransport.Handler
    getpoems grpctransport.Handler
}

func MakeGRPCServer(s poemProto.PoemServer) poemProto.PoemServer {
    return &grpcServer{
        addauthor: grpctransport.NewServer(
            MakeAddAuthorEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        delauthor: grpctransport.NewServer(
            MakeDelAuthorEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        updateauthor: grpctransport.NewServer(
            MakeUpdateAuthorEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        getoneauthor: grpctransport.NewServer(
            MakeGetOneAuthorEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        addpoem: grpctransport.NewServer(
            MakeAddPoemEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        delpoem: grpctransport.NewServer(
            MakeDelPoemEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        updatepoem: grpctransport.NewServer(
            MakeUpdatePoemEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        getonepoem: grpctransport.NewServer(
            MakeGetOnePoemEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
        getpoems: grpctransport.NewServer(
            MakeGetPoemsEndpoint(s),
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            func(_ context.Context, request interface{}) (interface{}, error) { return request, nil },
            ),
    }
}

func MakeAddAuthorEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.Author)
        return s.AddAuthor(ctx, req)
    }
    return ep
}

func (s *grpcServer) AddAuthor(ctx context.Context, req *poemProto.Author) (*poemProto.EmptyResponse, error) {
    _, rep, err := s.addauthor.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return rep.(*poemProto.EmptyResponse), nil
}

func MakeDelAuthorEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.Author)
        return s.DelAuthor(ctx, req)
    }
    return ep
}

func (s *grpcServer) DelAuthor(ctx context.Context, req *poemProto.Author) (*poemProto.EmptyResponse, error) {
    _, rep, err := s.delauthor.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return rep.(*poemProto.EmptyResponse), nil
}

func MakeUpdateAuthorEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.Author)
        return s.UpdateAuthor(ctx, req)
    }
    return ep
}

func (s *grpcServer) UpdateAuthor(ctx context.Context, req *poemProto.Author) (*poemProto.EmptyResponse, error) {
    _, rep, err := s.updateauthor.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return rep.(*poemProto.EmptyResponse), nil
}

func MakeGetOneAuthorEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.OneAuthorRequest)
        return s.GetOneAuthor(ctx, req)
    }
    return ep
}

func (s *grpcServer) GetOneAuthor(ctx context.Context, req *poemProto.OneAuthorRequest) (*poemProto.OneAuthorResponse, error) {
    _, rep, err := s.getoneauthor.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.OneAuthorResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}, Data:nil}, nil
    }
    return rep.(*poemProto.OneAuthorResponse), nil
}

func MakeAddPoemEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.PoemRecord)
        return s.AddPoem(ctx, req)
    }
    return ep
}

func (s *grpcServer) AddPoem(ctx context.Context, req *poemProto.PoemRecord) (*poemProto.EmptyResponse, error) {
    _, rep, err := s.addpoem.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return rep.(*poemProto.EmptyResponse), nil
}

func MakeDelPoemEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.PoemRecord)
        return s.DelPoem(ctx, req)
    }
    return ep
}

func (s *grpcServer) DelPoem(ctx context.Context, req *poemProto.PoemRecord) (*poemProto.EmptyResponse, error) {
    _, rep, err := s.delpoem.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return rep.(*poemProto.EmptyResponse), nil
}

func MakeUpdatePoemEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.PoemRecord)
        return s.UpdatePoem(ctx, req)
    }
    return ep
}

func (s *grpcServer) UpdatePoem(ctx context.Context, req *poemProto.PoemRecord) (*poemProto.EmptyResponse, error) {
    _, rep, err := s.updatepoem.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.EmptyResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}}, nil
    }
    return rep.(*poemProto.EmptyResponse), nil
}

func MakeGetOnePoemEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.OnePoemRequest)
        return s.GetOnePoem(ctx, req)
    }
    return ep
}

func (s *grpcServer) GetOnePoem(ctx context.Context, req *poemProto.OnePoemRequest) (*poemProto.OnePoemResponse, error) {
    _, rep, err := s.getonepoem.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.OnePoemResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}, Data:nil}, nil
    }
    return rep.(*poemProto.OnePoemResponse), nil
}

func MakeGetPoemsEndpoint(s poemProto.PoemServer) endpoint.Endpoint {
    ep := func(ctx context.Context, request interface{}) (response interface{}, err error) {
        req := request.(*poemProto.PoemsRequest)
        return s.GetPoems(ctx, req)
    }
    return ep
}

func (s *grpcServer) GetPoems(ctx context.Context, req *poemProto.PoemsRequest) (*poemProto.PoemsResponse, error) {
    _, rep, err := s.getpoems.ServeGRPC(ctx, req)
    if err != nil {
        return &poemProto.PoemsResponse{Stat:&poemProto.Status{Code:9999, Msg:err.Error()}, Data:nil}, nil
    }
    return rep.(*poemProto.PoemsResponse), nil
}
