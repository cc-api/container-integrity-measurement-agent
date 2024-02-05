module gosdk-test

go 1.20

require (
	github.com/golang/protobuf v1.5.3 // indirect
	github.com/intel/confidential-cloud-native-primitives/service/eventlog-server v0.0.0-20240131020930-fcd202dd676e // indirect
	github.com/pkg/errors v0.9.1 // indirect
	golang.org/x/net v0.18.0 // indirect
	golang.org/x/sys v0.14.0 // indirect
	golang.org/x/text v0.14.0 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20231106174013-bbf56f31fb17 // indirect
	google.golang.org/grpc v1.61.0 // indirect
	google.golang.org/protobuf v1.32.0 // indirect
)

require (
	ccnp v0.0.0-00010101000000-000000000000
	github.com/intel/confidential-cloud-native-primitives/sdk/golang/ccnp v0.0.0-20240131020930-fcd202dd676e
)

replace ccnp => ../../../sdk/golang/ccnp
