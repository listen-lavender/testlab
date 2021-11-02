// https://pkg.go.dev/google.golang.org/protobuf/types/dynamicpb

func readDynamically(in []byte) {
    registry, err := createProtoRegistry(".", "addressbook.proto")
    if err != nil {
        panic(err)
    }

    desc, err := registry.FindFileByPath("addressbook.proto")
    if err != nil {
        panic(err)
    }
    fd := desc.Messages()
    addressBook := fd.ByName("AddressBook")

    msg := dynamicpb.NewMessage(addressBook)
    err = proto.Unmarshal(in, msg)
    jsonBytes, err := protojson.Marshal(msg)
    if err != nil {
        panic(err)
    }
    fmt.Println(string(jsonBytes))
    if err != nil {
        panic(err)
    }
}

func createProtoRegistry(srcDir string, filename string) (*protoregistry.Files, error) {
    // Create descriptors using the protoc binary.
    // Imported dependencies are included so that the descriptors are self-contained.
    tmpFile := filename + "-tmp.pb"
    cmd := exec.Command("./protoc/protoc",
        "--include_imports",
        "--descriptor_set_out=" + tmpFile,
        "-I"+srcDir,
        path.Join(srcDir, filename))

    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    err := cmd.Run()
    if err != nil {
        return nil, err
    }
    defer os.Remove(tmpFile)

    marshalledDescriptorSet, err := ioutil.ReadFile(tmpFile)
    if err != nil {
        return nil, err
    }
    descriptorSet := descriptorpb.FileDescriptorSet{}
    err = proto.Unmarshal(marshalledDescriptorSet, &descriptorSet)
    if err != nil {
        return nil, err
    }

    files, err := protodesc.NewFiles(&descriptorSet)
    if err != nil {
        return nil, err
    }

    return files, nil
}
