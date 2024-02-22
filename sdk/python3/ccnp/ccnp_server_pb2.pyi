from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class HealthCheckRequest(_message.Message):
    __slots__ = ["service"]
    SERVICE_FIELD_NUMBER: _ClassVar[int]
    service: str
    def __init__(self, service: _Optional[str] = ...) -> None: ...

class HealthCheckResponse(_message.Message):
    __slots__ = ["status"]
    class ServingStatus(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
        UNKNOWN: _ClassVar[HealthCheckResponse.ServingStatus]
        SERVING: _ClassVar[HealthCheckResponse.ServingStatus]
        NOT_SERVING: _ClassVar[HealthCheckResponse.ServingStatus]
        SERVICE_UNKNOWN: _ClassVar[HealthCheckResponse.ServingStatus]
    UNKNOWN: HealthCheckResponse.ServingStatus
    SERVING: HealthCheckResponse.ServingStatus
    NOT_SERVING: HealthCheckResponse.ServingStatus
    SERVICE_UNKNOWN: HealthCheckResponse.ServingStatus
    STATUS_FIELD_NUMBER: _ClassVar[int]
    status: HealthCheckResponse.ServingStatus
    def __init__(self, status: _Optional[_Union[HealthCheckResponse.ServingStatus, str]] = ...) -> None: ...

class GetDefaultAlgorithmRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class GetDefaultAlgorithmResponse(_message.Message):
    __slots__ = ["algo_id"]
    ALGO_ID_FIELD_NUMBER: _ClassVar[int]
    algo_id: int
    def __init__(self, algo_id: _Optional[int] = ...) -> None: ...

class GetMeasurementCountRequest(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class GetMeasurementCountResponse(_message.Message):
    __slots__ = ["count"]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    count: int
    def __init__(self, count: _Optional[int] = ...) -> None: ...

class GetCcReportRequest(_message.Message):
    __slots__ = ["container_id", "user_data", "nonce"]
    CONTAINER_ID_FIELD_NUMBER: _ClassVar[int]
    USER_DATA_FIELD_NUMBER: _ClassVar[int]
    NONCE_FIELD_NUMBER: _ClassVar[int]
    container_id: str
    user_data: str
    nonce: str
    def __init__(self, container_id: _Optional[str] = ..., user_data: _Optional[str] = ..., nonce: _Optional[str] = ...) -> None: ...

class GetCcReportResponse(_message.Message):
    __slots__ = ["cc_type", "cc_report"]
    CC_TYPE_FIELD_NUMBER: _ClassVar[int]
    CC_REPORT_FIELD_NUMBER: _ClassVar[int]
    cc_type: int
    cc_report: bytes
    def __init__(self, cc_type: _Optional[int] = ..., cc_report: _Optional[bytes] = ...) -> None: ...

class GetCcMeasurementRequest(_message.Message):
    __slots__ = ["container_id", "index", "algo_id"]
    CONTAINER_ID_FIELD_NUMBER: _ClassVar[int]
    INDEX_FIELD_NUMBER: _ClassVar[int]
    ALGO_ID_FIELD_NUMBER: _ClassVar[int]
    container_id: str
    index: int
    algo_id: int
    def __init__(self, container_id: _Optional[str] = ..., index: _Optional[int] = ..., algo_id: _Optional[int] = ...) -> None: ...

class GetCcMeasurementResponse(_message.Message):
    __slots__ = ["measurement"]
    MEASUREMENT_FIELD_NUMBER: _ClassVar[int]
    measurement: TcgDigest
    def __init__(self, measurement: _Optional[_Union[TcgDigest, _Mapping]] = ...) -> None: ...

class GetCcEventlogRequest(_message.Message):
    __slots__ = ["container_id", "start", "count"]
    CONTAINER_ID_FIELD_NUMBER: _ClassVar[int]
    START_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    container_id: str
    start: int
    count: int
    def __init__(self, container_id: _Optional[str] = ..., start: _Optional[int] = ..., count: _Optional[int] = ...) -> None: ...

class TcgDigest(_message.Message):
    __slots__ = ["algo_id", "hash"]
    ALGO_ID_FIELD_NUMBER: _ClassVar[int]
    HASH_FIELD_NUMBER: _ClassVar[int]
    algo_id: int
    hash: bytes
    def __init__(self, algo_id: _Optional[int] = ..., hash: _Optional[bytes] = ...) -> None: ...

class TcgEventlog(_message.Message):
    __slots__ = ["rec_num", "imr_index", "event_type", "digests", "event_size", "event", "extra_info"]
    class ExtraInfoEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    REC_NUM_FIELD_NUMBER: _ClassVar[int]
    IMR_INDEX_FIELD_NUMBER: _ClassVar[int]
    EVENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    DIGESTS_FIELD_NUMBER: _ClassVar[int]
    EVENT_SIZE_FIELD_NUMBER: _ClassVar[int]
    EVENT_FIELD_NUMBER: _ClassVar[int]
    EXTRA_INFO_FIELD_NUMBER: _ClassVar[int]
    rec_num: int
    imr_index: int
    event_type: int
    digests: _containers.RepeatedCompositeFieldContainer[TcgDigest]
    event_size: int
    event: bytes
    extra_info: _containers.ScalarMap[str, str]
    def __init__(self, rec_num: _Optional[int] = ..., imr_index: _Optional[int] = ..., event_type: _Optional[int] = ..., digests: _Optional[_Iterable[_Union[TcgDigest, _Mapping]]] = ..., event_size: _Optional[int] = ..., event: _Optional[bytes] = ..., extra_info: _Optional[_Mapping[str, str]] = ...) -> None: ...

class GetCcEventlogResponse(_message.Message):
    __slots__ = ["event_logs"]
    EVENT_LOGS_FIELD_NUMBER: _ClassVar[int]
    event_logs: _containers.RepeatedCompositeFieldContainer[TcgEventlog]
    def __init__(self, event_logs: _Optional[_Iterable[_Union[TcgEventlog, _Mapping]]] = ...) -> None: ...
