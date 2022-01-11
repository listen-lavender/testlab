package main

func LoadP12TLSCfg(keystore, password string) (*x509.CertPool, tls.Certificate, error) {
    data, err := ioutil.ReadFile(keystore)
    if err != nil {
        return nil, tls.Certificate{}, err

    }
    pk, crt, caCrts, err := pkcs12.DecodeChain(data, password)
    if err != nil {
        return nil, tls.Certificate{}, err
    }
    pool := x509.NewCertPool()
    pool.AddCert(caCrts[0])
    tlsCrt := tls.Certificate{
        Certificate: [][]byte{crt.Raw},
        Leaf:        crt,
        PrivateKey:  pk,
    }
    return pool, tlsCrt, nil
}

func LoadServerTLSCfg(keystore, password string) (*tls.Config, error) {
    pool, crt, err := LoadP12TLSCfg(keystore, password)
    if err != nil {
        return nil, err
    }
    cfg := &tls.Config{
        ClientCAs:    pool,
        ClientAuth:   tls.RequireAndVerifyClientCert,
        Certificates: []tls.Certificate{crt},
    }
    return cfg, nil
}

func LoadClientTLSCfg(keystore, password string, serverName string) (*tls.Config, error) {
    pool, crt, err := LoadP12TLSCfg(keystore, password)
    if err != nil {
        return nil, err
    }
    cfg := &tls.Config{
        RootCAs:      pool,
        Certificates: []tls.Certificate{crt},
        ServerName:   serverName,
    }
    return cfg, nil
}

func main() {
    
}