#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequest {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckResponse {
    #[prost(enumeration = "health_check_response::ServingStatus", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `HealthCheckResponse`.
pub mod health_check_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ServingStatus {
        Unknown = 0,
        Serving = 1,
        NotServing = 2,
        ServiceUnknown = 3,
    }
    impl ServingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServingStatus::Unknown => "UNKNOWN",
                ServingStatus::Serving => "SERVING",
                ServingStatus::NotServing => "NOT_SERVING",
                ServingStatus::ServiceUnknown => "SERVICE_UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SERVING" => Some(Self::Serving),
                "NOT_SERVING" => Some(Self::NotServing),
                "SERVICE_UNKNOWN" => Some(Self::ServiceUnknown),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultAlgorithmRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultAlgorithmResponse {
    #[prost(uint32, tag = "1")]
    pub algo_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeasurementCountRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeasurementCountResponse {
    #[prost(uint32, tag = "1")]
    pub count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCcReportRequest {
    #[prost(string, tag = "1")]
    pub user_data: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub nonce: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCcReportResponse {
    #[prost(int32, tag = "1")]
    pub cc_type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub cc_report: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCcMeasurementRequest {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(uint32, tag = "2")]
    pub algo_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCcMeasurementResponse {
    #[prost(message, optional, tag = "1")]
    pub measurement: ::core::option::Option<TcgDigest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCcEventlogRequest {
    #[prost(uint32, tag = "1")]
    pub start: u32,
    #[prost(uint32, tag = "2")]
    pub count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcgDigest {
    #[prost(uint32, tag = "1")]
    pub algo_id: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcgEventlog {
    #[prost(uint32, tag = "1")]
    pub rec_num: u32,
    #[prost(uint32, tag = "2")]
    pub imr_index: u32,
    #[prost(uint32, tag = "3")]
    pub event_type: u32,
    #[prost(message, repeated, tag = "4")]
    pub digests: ::prost::alloc::vec::Vec<TcgDigest>,
    #[prost(uint32, tag = "5")]
    pub event_size: u32,
    #[prost(bytes = "vec", tag = "6")]
    pub event: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "7")]
    pub extra_info: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCcEventlogResponse {
    #[prost(message, repeated, tag = "1")]
    pub event_logs: ::prost::alloc::vec::Vec<TcgEventlog>,
}
/// Generated client implementations.
pub mod ccnp_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CcnpClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CcnpClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CcnpClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CcnpClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CcnpClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_default_algorithm(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultAlgorithmRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultAlgorithmResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccnp_server_pb.ccnp/GetDefaultAlgorithm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ccnp_server_pb.ccnp", "GetDefaultAlgorithm"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_measurement_count(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMeasurementCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMeasurementCountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccnp_server_pb.ccnp/GetMeasurementCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ccnp_server_pb.ccnp", "GetMeasurementCount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cc_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCcReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCcReportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccnp_server_pb.ccnp/GetCcReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ccnp_server_pb.ccnp", "GetCcReport"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cc_measurement(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCcMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCcMeasurementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccnp_server_pb.ccnp/GetCcMeasurement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ccnp_server_pb.ccnp", "GetCcMeasurement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cc_eventlog(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCcEventlogRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCcEventlogResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccnp_server_pb.ccnp/GetCcEventlog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ccnp_server_pb.ccnp", "GetCcEventlog"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ccnp_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CcnpServer.
    #[async_trait]
    pub trait Ccnp: Send + Sync + 'static {
        async fn get_default_algorithm(
            &self,
            request: tonic::Request<super::GetDefaultAlgorithmRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultAlgorithmResponse>,
            tonic::Status,
        >;
        async fn get_measurement_count(
            &self,
            request: tonic::Request<super::GetMeasurementCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMeasurementCountResponse>,
            tonic::Status,
        >;
        async fn get_cc_report(
            &self,
            request: tonic::Request<super::GetCcReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCcReportResponse>,
            tonic::Status,
        >;
        async fn get_cc_measurement(
            &self,
            request: tonic::Request<super::GetCcMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCcMeasurementResponse>,
            tonic::Status,
        >;
        async fn get_cc_eventlog(
            &self,
            request: tonic::Request<super::GetCcEventlogRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCcEventlogResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CcnpServer<T: Ccnp> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Ccnp> CcnpServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CcnpServer<T>
    where
        T: Ccnp,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ccnp_server_pb.ccnp/GetDefaultAlgorithm" => {
                    #[allow(non_camel_case_types)]
                    struct GetDefaultAlgorithmSvc<T: Ccnp>(pub Arc<T>);
                    impl<
                        T: Ccnp,
                    > tonic::server::UnaryService<super::GetDefaultAlgorithmRequest>
                    for GetDefaultAlgorithmSvc<T> {
                        type Response = super::GetDefaultAlgorithmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDefaultAlgorithmRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_default_algorithm(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDefaultAlgorithmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ccnp_server_pb.ccnp/GetMeasurementCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetMeasurementCountSvc<T: Ccnp>(pub Arc<T>);
                    impl<
                        T: Ccnp,
                    > tonic::server::UnaryService<super::GetMeasurementCountRequest>
                    for GetMeasurementCountSvc<T> {
                        type Response = super::GetMeasurementCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMeasurementCountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_measurement_count(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMeasurementCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ccnp_server_pb.ccnp/GetCcReport" => {
                    #[allow(non_camel_case_types)]
                    struct GetCcReportSvc<T: Ccnp>(pub Arc<T>);
                    impl<T: Ccnp> tonic::server::UnaryService<super::GetCcReportRequest>
                    for GetCcReportSvc<T> {
                        type Response = super::GetCcReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCcReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_cc_report(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCcReportSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ccnp_server_pb.ccnp/GetCcMeasurement" => {
                    #[allow(non_camel_case_types)]
                    struct GetCcMeasurementSvc<T: Ccnp>(pub Arc<T>);
                    impl<
                        T: Ccnp,
                    > tonic::server::UnaryService<super::GetCcMeasurementRequest>
                    for GetCcMeasurementSvc<T> {
                        type Response = super::GetCcMeasurementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCcMeasurementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_cc_measurement(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCcMeasurementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ccnp_server_pb.ccnp/GetCcEventlog" => {
                    #[allow(non_camel_case_types)]
                    struct GetCcEventlogSvc<T: Ccnp>(pub Arc<T>);
                    impl<
                        T: Ccnp,
                    > tonic::server::UnaryService<super::GetCcEventlogRequest>
                    for GetCcEventlogSvc<T> {
                        type Response = super::GetCcEventlogResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCcEventlogRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_cc_eventlog(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCcEventlogSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Ccnp> Clone for CcnpServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Ccnp> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Ccnp> tonic::server::NamedService for CcnpServer<T> {
        const NAME: &'static str = "ccnp_server_pb.ccnp";
    }
}
