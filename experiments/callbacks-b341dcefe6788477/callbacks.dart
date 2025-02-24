library callbacks;

import "dart:async";
import "dart:convert";
import "dart:ffi";
import "dart:io" show Platform, File, Directory;
import "dart:typed_data";
import "package:ffi/ffi.dart";

abstract class ComplexException {
  RustBuffer lower();
  int allocationSize();
  int write(Uint8List buf);
}

class FfiConverterComplexException {
  static ComplexException lift(RustBuffer buffer) {
    return FfiConverterComplexException.read(buffer.asUint8List()).value;
  }

  static LiftRetVal<ComplexException> read(Uint8List buf) {
    final index = buf.buffer.asByteData(buf.offsetInBytes).getInt32(0);
    final subview = Uint8List.view(buf.buffer, buf.offsetInBytes + 4);
    switch (index) {
      case 1:
        return ReallyBadArgumentComplexException.read(subview);
      case 2:
        return UnexpectedExceptionWithReasonComplexException.read(subview);
      default:
        throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase,
            "Unable to determine enum variant");
    }
  }

  static RustBuffer lower(ComplexException value) {
    return value.lower();
  }

  static int allocationSize(ComplexException value) {
    return value.allocationSize();
  }

  static int write(ComplexException value, Uint8List buf) {
    return value.write(buf);
  }
}

class ReallyBadArgumentComplexException extends ComplexException {
  final int code;

  ReallyBadArgumentComplexException({required this.code});

  ReallyBadArgumentComplexException._(
    this.code,
  );

  static LiftRetVal<ReallyBadArgumentComplexException> read(Uint8List buf) {
    int new_offset = buf.offsetInBytes;

    final code_lifted =
        FfiConverterInt32.read(Uint8List.view(buf.buffer, new_offset));
    final code = code_lifted.value;
    new_offset += code_lifted.bytesRead;
    return LiftRetVal(
        ReallyBadArgumentComplexException._(
          code,
        ),
        new_offset);
  }

  @override
  RustBuffer lower() {
    final buf = Uint8List(allocationSize());
    write(buf);
    return toRustBuffer(buf);
  }

  @override
  int allocationSize() {
    return FfiConverterInt32.allocationSize(code) + 4;
  }

  @override
  int write(Uint8List buf) {
    buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, 1);
    int new_offset = buf.offsetInBytes + 4;

    new_offset +=
        FfiConverterInt32.write(code, Uint8List.view(buf.buffer, new_offset));

    return new_offset;
  }
}

class UnexpectedExceptionWithReasonComplexException extends ComplexException {
  final String reason;

  UnexpectedExceptionWithReasonComplexException({required this.reason});

  UnexpectedExceptionWithReasonComplexException._(
    this.reason,
  );

  static LiftRetVal<UnexpectedExceptionWithReasonComplexException> read(
      Uint8List buf) {
    int new_offset = buf.offsetInBytes;

    final reason_lifted =
        FfiConverterString.read(Uint8List.view(buf.buffer, new_offset));
    final reason = reason_lifted.value;
    new_offset += reason_lifted.bytesRead;
    return LiftRetVal(
        UnexpectedExceptionWithReasonComplexException._(
          reason,
        ),
        new_offset);
  }

  @override
  RustBuffer lower() {
    final buf = Uint8List(allocationSize());
    write(buf);
    return toRustBuffer(buf);
  }

  @override
  int allocationSize() {
    return FfiConverterString.allocationSize(reason) + 4;
  }

  @override
  int write(Uint8List buf) {
    buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, 2);
    int new_offset = buf.offsetInBytes + 4;

    new_offset += FfiConverterString.write(
        reason, Uint8List.view(buf.buffer, new_offset));

    return new_offset;
  }
}

enum SimpleException {
  BadArgument,
  UnexpectedException,
}

class FfiConverterSimpleException {
  static SimpleException lift(RustBuffer buffer) {
    final index = buffer.asUint8List().buffer.asByteData().getInt32(0);
    switch (index) {
      case 1:
        return SimpleException.BadArgument;
      case 2:
        return SimpleException.UnexpectedException;
      default:
        throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase,
            "Unable to determine enum variant");
    }
  }

  static RustBuffer lower(SimpleException input) {
    return toRustBuffer(createUint8ListFromInt(input.index + 1));
  }
}

final _RustGettersFinalizer = Finalizer<Pointer<Void>>((ptr) {
  rustCall((status) =>
      _UniffiLib.instance.uniffi_callbacks_fn_free_rustgetters(ptr, status));
});

class RustGetters {
  late final Pointer<Void> _ptr;

  RustGetters() {
    this._ptr = rustCall((status) => _UniffiLib.instance
        .uniffi_callbacks_fn_constructor_rustgetters_new(status));
    _RustGettersFinalizer.attach(this, _ptr, detach: this);
  }

  RustGetters._(this._ptr) {
    _RustGettersFinalizer.attach(this, _ptr, detach: this);
  }

  factory RustGetters.lift(Pointer<Void> ptr) {
    return RustGetters._(ptr);
  }

  Pointer<Void> uniffiClonePointer() {
    return rustCall((status) => _UniffiLib.instance
        .uniffi_callbacks_fn_clone_rustgetters(_ptr, status));
  }

  void dispose() {
    _RustGettersFinalizer.detach(this);
    rustCall((status) =>
        _UniffiLib.instance.uniffi_callbacks_fn_free_rustgetters(_ptr, status));
  }

  bool getBool(
    ForeignGetters callback,
    bool v,
    bool argumentTwo,
  ) {
    var loweredCallback = FfiConverterCallbackInterfaceForeignGetters.lower(callback);
    var loweredV = FfiConverterBool.lower(v);
    var loweredArgumentTwo = FfiConverterBool.lower(argumentTwo);
    return rustCall((status) => FfiConverterBool.lift(_UniffiLib.instance
        .uniffi_callbacks_fn_method_rustgetters_get_bool(
            uniffiClonePointer(),
            loweredCallback,
            loweredV,
            loweredArgumentTwo,
            status)));
  }

  List<int> getList(
    ForeignGetters callback,
    List<int> v,
    bool arg2,
  ) {
    var self = uniffiClonePointer();
    var call = FfiConverterCallbackInterfaceForeignGetters.lower(callback);
    var sequence = FfiConverterSequenceInt32.lower(v);
    var arguemtn2 = FfiConverterBool.lower(arg2);

    return rustCall((status) => FfiConverterSequenceInt32.lift(
        _UniffiLib.instance.uniffi_callbacks_fn_method_rustgetters_get_list(
            self,
            call,
            sequence,
            arguemtn2,
            status)));
  }

  void getNothing(
    ForeignGetters callback,
    String v,
  ) {
    return rustCall((status) {
      _UniffiLib.instance.uniffi_callbacks_fn_method_rustgetters_get_nothing(
        uniffiClonePointer(),
        FfiConverterCallbackInterfaceForeignGetters.lower(callback),
        FfiConverterString.lower(v),
        status,
      );
      // No return needed if 'rustCall' expects a void callback or we can return null if needed.
    });
  }

  String? getOption(
    ForeignGetters callback,
    String? v,
    bool arg2,
  ) {
    print('Rust Getters getOption');
    var self = uniffiClonePointer();
    var call = FfiConverterCallbackInterfaceForeignGetters.lower(callback);
    var arguemtn2 = FfiConverterBool.lower(arg2);
    var tempv = FfiConverterOptionalString.lower(v);

    print("Passing uniffi_callbacks_fn_method_rustgetters_get_string_optional_callback");
    return rustCall((status) => FfiConverterOptionalString.lift(
        _UniffiLib.instance.uniffi_callbacks_fn_method_rustgetters_get_option(
            uniffiClonePointer(),
            FfiConverterCallbackInterfaceForeignGetters.lower(callback),
            FfiConverterOptionalString.lower(v),
            FfiConverterBool.lower(arg2),
            status)));
  }

  String getString(
    ForeignGetters callback,
    String v,
    bool arg2,
  ) {
    return rustCall((status) => FfiConverterString.lift(_UniffiLib.instance
        .uniffi_callbacks_fn_method_rustgetters_get_string(
            uniffiClonePointer(),
            FfiConverterCallbackInterfaceForeignGetters.lower(callback),
            FfiConverterString.lower(v),
            FfiConverterBool.lower(arg2),
            status)));
  }

  String? getStringOptionalCallback(
    ForeignGetters? callback,
    String v,
    bool arg2,
  ) {
    print("Passing uniffi_callbacks_fn_method_rustgetters_get_string_optional_callback");
    return rustCall((status) => FfiConverterOptionalString.lift(_UniffiLib
        .instance
        .uniffi_callbacks_fn_method_rustgetters_get_string_optional_callback(
            uniffiClonePointer(),
            FfiConverterOptionalCallbackInterfaceForeignGetters.lower(callback),
            FfiConverterString.lower(v),
            FfiConverterBool.lower(arg2),
            status)));
  }
}

final _RustStringifierFinalizer = Finalizer<Pointer<Void>>((ptr) {
  rustCall((status) => _UniffiLib.instance
      .uniffi_callbacks_fn_free_ruststringifier(ptr, status));
});

class RustStringifier {
  late final Pointer<Void> _ptr;

  RustStringifier(StoredForeignStringifier callback) {
    this._ptr = rustCall((status) => _UniffiLib.instance
        .uniffi_callbacks_fn_constructor_ruststringifier_new(
            FfiConverterCallbackInterfaceStoredForeignStringifier.lower(
                callback),
            status));
    _RustStringifierFinalizer.attach(this, _ptr, detach: this);
  }

  RustStringifier._(this._ptr) {
    _RustStringifierFinalizer.attach(this, _ptr, detach: this);
  }

  factory RustStringifier.lift(Pointer<Void> ptr) {
    return RustStringifier._(ptr);
  }

  Pointer<Void> uniffiClonePointer() {
    return rustCall((status) => _UniffiLib.instance
        .uniffi_callbacks_fn_clone_ruststringifier(_ptr, status));
  }

  void dispose() {
    _RustStringifierFinalizer.detach(this);
    rustCall((status) => _UniffiLib.instance
        .uniffi_callbacks_fn_free_ruststringifier(_ptr, status));
  }

  String fromSimpleType(
    int value,
  ) {
    return rustCall((status) => FfiConverterString.lift(_UniffiLib.instance
        .uniffi_callbacks_fn_method_ruststringifier_from_simple_type(
            uniffiClonePointer(), value, status)));
  }
}

class UniffiInternalError implements Exception {
  static const int bufferOverflow = 0;
  static const int incompleteData = 1;
  static const int unexpectedOptionalTag = 2;
  static const int unexpectedEnumCase = 3;
  static const int unexpectedNullPointer = 4;
  static const int unexpectedRustCallStatusCode = 5;
  static const int unexpectedRustCallError = 6;
  static const int unexpectedStaleHandle = 7;
  static const int rustPanic = 8;

  final int errorCode;
  final String? panicMessage;

  const UniffiInternalError(this.errorCode, this.panicMessage);

  static UniffiInternalError panicked(String message) {
    return UniffiInternalError(rustPanic, message);
  }

  @override
  String toString() {
    switch (errorCode) {
      case bufferOverflow:
        return "UniFfi::BufferOverflow";
      case incompleteData:
        return "UniFfi::IncompleteData";
      case unexpectedOptionalTag:
        return "UniFfi::UnexpectedOptionalTag";
      case unexpectedEnumCase:
        return "UniFfi::UnexpectedEnumCase";
      case unexpectedNullPointer:
        return "UniFfi::UnexpectedNullPointer";
      case unexpectedRustCallStatusCode:
        return "UniFfi::UnexpectedRustCallStatusCode";
      case unexpectedRustCallError:
        return "UniFfi::UnexpectedRustCallError";
      case unexpectedStaleHandle:
        return "UniFfi::UnexpectedStaleHandle";
      case rustPanic:
        return "UniFfi::rustPanic: \$\$panicMessage";
      default:
        return "UniFfi::UnknownError: \$\$errorCode";
    }
  }
}

const int CALL_SUCCESS = 0;
const int CALL_ERROR = 1;
const int CALL_UNEXPECTED_ERROR = 2;

final class RustCallStatus extends Struct {
  @Int8()
  external int code;

  external RustBuffer errorBuf;
}

void checkCallStatus(UniffiRustCallStatusErrorHandler errorHandler,
    Pointer<RustCallStatus> status) {
  if (status.ref.code == CALL_SUCCESS) {
    return;
  } else if (status.ref.code == CALL_ERROR) {
    throw errorHandler.lift(status.ref.errorBuf);
  } else if (status.ref.code == CALL_UNEXPECTED_ERROR) {
    if (status.ref.errorBuf.len > 0) {
      throw UniffiInternalError.panicked(
          FfiConverterString.lift(status.ref.errorBuf));
    } else {
      throw UniffiInternalError.panicked("Rust panic");
    }
  } else {
    throw UniffiInternalError.panicked(
        "Unexpected RustCallStatus code: \${status.code}");
  }
}

T rustCall<T>(T Function(Pointer<RustCallStatus>) callback) {
  final status = calloc<RustCallStatus>();
  try {
    return callback(status);
  } finally {
    calloc.free(status);
  }
}

class NullRustCallStatusErrorHandler extends UniffiRustCallStatusErrorHandler {
  @override
  Exception lift(RustBuffer errorBuf) {
    errorBuf.free();
    return UniffiInternalError.panicked("Unexpected CALL_ERROR");
  }
}

abstract class UniffiRustCallStatusErrorHandler {
  Exception lift(RustBuffer errorBuf);
}

final class RustBuffer extends Struct {
  @Uint64()
  external int capacity;

  @Uint64()
  external int len;

  external Pointer<Uint8> data;

  static RustBuffer alloc(int size) {
    return rustCall((status) =>
        _UniffiLib.instance.ffi_callbacks_rustbuffer_alloc(size, status));
  }

  static RustBuffer fromBytes(ForeignBytes bytes) {
    return rustCall((status) =>
        _UniffiLib.instance.ffi_callbacks_rustbuffer_from_bytes(bytes, status));
  }

  void free() {
    rustCall((status) =>
        _UniffiLib.instance.ffi_callbacks_rustbuffer_free(this, status));
  }

  RustBuffer reserve(int additionalCapacity) {
    return rustCall((status) => _UniffiLib.instance
        .ffi_callbacks_rustbuffer_reserve(this, additionalCapacity, status));
  }

  Uint8List asUint8List() {
    final dataList = data.asTypedList(len);
    final byteData = ByteData.sublistView(dataList);
    return Uint8List.view(byteData.buffer);
  }

  @override
  String toString() {
    return "RustBuffer{capacity: $capacity, len: $len, data: $data}";
  }
}

RustBuffer toRustBuffer(Uint8List data) {
  final length = data.length;

  final Pointer<Uint8> frameData = calloc<Uint8>(length);
  final pointerList = frameData.asTypedList(length);
  pointerList.setAll(0, data);

  final bytes = calloc<ForeignBytes>();
  bytes.ref.len = length;
  bytes.ref.data = frameData;
  return RustBuffer.fromBytes(bytes.ref);
}

final class ForeignBytes extends Struct {
  @Int32()
  external int len;
  external Pointer<Uint8> data;

  void free() {
    calloc.free(data);
  }
}

class LiftRetVal<T> {
  final T value;
  final int bytesRead;
  const LiftRetVal(this.value, this.bytesRead);

  LiftRetVal<T> copyWithOffset(int offset) {
    return LiftRetVal(value, bytesRead + offset);
  }
}

abstract class FfiConverter<D, F> {
  const FfiConverter();

  D lift(F value);
  F lower(D value);
  D read(ByteData buffer, int offset);
  void write(D value, ByteData buffer, int offset);
  int size(D value);
}

mixin FfiConverterPrimitive<T> on FfiConverter<T, T> {
  @override
  T lift(T value) => value;

  @override
  T lower(T value) => value;
}

Uint8List createUint8ListFromInt(int value) {
  int length = value.bitLength ~/ 8 + 1;

  if (length != 4 && length != 8) {
    length = (value < 0x100000000) ? 4 : 8;
  }

  Uint8List uint8List = Uint8List(length);

  for (int i = length - 1; i >= 0; i--) {
    uint8List[i] = value & 0xFF;
    value >>= 8;
  }

  return uint8List;
}

class FfiConverterOptionalCallbackInterfaceForeignGetters {
  static ForeignGetters? lift(RustBuffer buf) {
    return FfiConverterOptionalCallbackInterfaceForeignGetters.read(
            buf.asUint8List())
        .value;
  }

  static LiftRetVal<ForeignGetters?> read(Uint8List buf) {
    if (ByteData.view(buf.buffer, buf.offsetInBytes).getInt8(0) == 0) {
      return LiftRetVal(null, 1);
    }
    return FfiConverterCallbackInterfaceForeignGetters.read(
            Uint8List.view(buf.buffer, buf.offsetInBytes + 1))
        .copyWithOffset(1);
  }

  static int allocationSize([ForeignGetters? value]) {
    if (value == null) {
      return 1;
    }
    return FfiConverterCallbackInterfaceForeignGetters.allocationSize(value) +
        1;
  }

  static RustBuffer lower(ForeignGetters? value) {
    if (value == null) {
      return toRustBuffer(Uint8List.fromList([0]));
    }

    final length =
        FfiConverterOptionalCallbackInterfaceForeignGetters.allocationSize(
            value);

    final Pointer<Uint8> frameData = calloc<Uint8>(length);
    final buf = frameData.asTypedList(length);

    FfiConverterOptionalCallbackInterfaceForeignGetters.write(value, buf);

    final bytes = calloc<ForeignBytes>();
    bytes.ref.len = length;
    bytes.ref.data = frameData;
    return RustBuffer.fromBytes(bytes.ref);
  }

  static int write(ForeignGetters? value, Uint8List buf) {
    if (value == null) {
      buf[0] = 0;
      return 1;
    }

    buf[0] = 1;

    return FfiConverterCallbackInterfaceForeignGetters.write(
            value, Uint8List.view(buf.buffer, buf.offsetInBytes + 1)) +
        1;
  }
}

class FfiConverterOptionalString {
  static String? lift(RustBuffer buf) {
    print("Buffer Read when lifting: $buf");
    return FfiConverterOptionalString.read(buf.asUint8List()).value;
  }

  static LiftRetVal<String?> read(Uint8List buf) {
    var tempbuf = buf;
    if (ByteData.view(buf.buffer, buf.offsetInBytes).getInt8(0) == 0) {
      return LiftRetVal(null, 1);
    }
    print("Uint8List.view(buf.buffer, buf.offsetInBytes + 1): ${Uint8List.view(buf.buffer, buf.offsetInBytes + 1)}");
    return FfiConverterString.read(
            Uint8List.view(buf.buffer, buf.offsetInBytes + 1))
        .copyWithOffset(1);
  }

  static int allocationSize([String? value]) {
    if (value == null) {
      return 1;
    }
    return FfiConverterString.allocationSize(value) + 1;
  }

  static RustBuffer lower(String? value) {
    if (value == null) {
      return toRustBuffer(Uint8List.fromList([0]));
    }

    final length = FfiConverterOptionalString.allocationSize(value);

    final Pointer<Uint8> frameData = calloc<Uint8>(length);
    final buf = frameData.asTypedList(length);

    FfiConverterOptionalString.write(value, buf);

    final bytes = calloc<ForeignBytes>();
    bytes.ref.len = length;
    bytes.ref.data = frameData;
    return RustBuffer.fromBytes(bytes.ref);
  }

  static int write(String? value, Uint8List buf) {
    if (value == null) {
      buf[0] = 0;
      return 1;
    }

    buf[0] = 1;

    return FfiConverterString.write(
            value, Uint8List.view(buf.buffer, buf.offsetInBytes + 1)) +
        1;
  }
}

class FfiConverterString {
  static String lift(RustBuffer buf) {
    return utf8.decoder.convert(buf.asUint8List());
  }

  static RustBuffer lower(String value) {
    return toRustBuffer(Utf8Encoder().convert(value));
  }

  static LiftRetVal<String> read(Uint8List buf) {
    final end = buf.buffer.asByteData(buf.offsetInBytes).getInt32(0) + 4;
    return LiftRetVal(utf8.decoder.convert(buf, 4, end), end);
  }

  static int allocationSize([String value = ""]) {
    return utf8.encoder.convert(value).length + 4;
  }

  static int write(String value, Uint8List buf) {
    final list = utf8.encoder.convert(value);
    buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, list.length);
    buf.setAll(4, list);
    return list.length + 4;
  }
}

class FfiConverterDoubleNullable {
  static List<double?>? lift(RustBuffer buf) {
    return read(buf.asUint8List()).value;
  }

  static LiftRetVal<List<double?>?> read(Uint8List buf) {
    if (ByteData.view(buf.buffer, buf.offsetInBytes).getInt8(0) == 0) {
      return LiftRetVal(null, 1);
    }
    // Implement the logic to read List<double?> from buffer
    // This is a placeholder and should match your Rust side data layout
    // For example:
    final length = ByteData.view(buf.buffer, buf.offsetInBytes + 1).getInt32(0);
    List<double?> res = [];
    int offset = buf.offsetInBytes + 5;
    for (var i = 0; i < length; i++) {
      final hasValue = ByteData.view(buf.buffer, offset).getInt8(0) == 1;
      offset += 1;
      if (hasValue) {
        final value = ByteData.view(buf.buffer, offset).getFloat64(0);
        res.add(value);
        offset += 8;
      } else {
        res.add(null);
      }
    }
    return LiftRetVal(res, offset - buf.offsetInBytes);
  }

  static int allocationSize([List<double?>? value]) {
    if (value == null) {
      return 1;
    }
    int size = 1 + 4; // null flag + length
    for (var item in value) {
      size += 1; // presence flag
      if (item != null) {
        size += 8; // double size
      }
    }
    return size;
  }

  static RustBuffer lower(List<double?>? value) {
    final size = allocationSize(value);
    final buf = Uint8List(size);
    if (value == null) {
      buf[0] = 0;
      return toRustBuffer(buf);
    }
    buf[0] = 1;
    buf.buffer.asByteData().setInt32(1, value.length);
    int offset = 5;
    for (var item in value) {
      if (item == null) {
        buf[offset] = 0;
        offset += 1;
      } else {
        buf[offset] = 1;
        offset += 1;
        buf.buffer.asByteData().setFloat64(offset, item);
        offset += 8;
      }
    }
    return toRustBuffer(buf);
  }

  static int write(List<double?>? value, Uint8List buf) {
    if (value == null) {
      buf[0] = 0;
      return 1;
    }
    buf[0] = 1;
    buf.buffer.asByteData().setInt32(1, value.length);
    int offset = 5;
    for (var item in value) {
      if (item == null) {
        buf[offset] = 0;
        offset += 1;
      } else {
        buf[offset] = 1;
        offset += 1;
        buf.buffer.asByteData().setFloat64(offset, item);
        offset += 8;
      }
    }
    return offset;
  }
}

// "Plese start here"

// You may need to ensureInitialized() on library load to verify checksums/versioning.

/// A handle map to store callback instances passed from Dart to Rust.
class UniffiHandleMap<T> {
  final Map<int, T> _map = {};
  int _counter = 0;

  int insert(T obj) {
    final handle = _counter++;
    _map[handle] = obj;
    return handle;
  }

  T get(int handle) {
    final obj = _map[handle];
    if (obj == null) {
      throw UniffiInternalError(
          UniffiInternalError.unexpectedStaleHandle, "Handle not found");
    }
    return obj;
  }

  void remove(int handle) {
    if (_map.remove(handle) == null) {
      throw UniffiInternalError(
          UniffiInternalError.unexpectedStaleHandle, "Handle not found");
    }
  }
}

/// ForeignGetters is implemented by the foreign language (Dart in this case), and Rust calls methods on it.
abstract class ForeignGetters {
  bool getBool(bool v, bool argumentTwo);
  String getString(String v, bool arg2);
  String? getOption(String? v, bool arg2);
  List<int> getList(List<int> v, bool arg2);
  void getNothing(String v);
}

/// StoredForeignStringifier interface.
abstract class StoredForeignStringifier {
  String fromSimpleType(int value);
  String fromComplexType(List<double?>? values);
}

// We need converters for these callback interfaces.

/// Converter for ForeignGetters callback interface.
class FfiConverterCallbackInterfaceForeignGetters {
  static final _handleMap = UniffiHandleMap<ForeignGetters>();

  static ForeignGetters lift(int handle) {
    return _handleMap.get(handle);
  }

  static int lower(ForeignGetters value) {
    return _handleMap.insert(value);
  }

  static LiftRetVal<ForeignGetters> read(Uint8List buf) {
    final handle = buf.buffer.asByteData(buf.offsetInBytes).getInt64(0);
    return LiftRetVal(lift(handle), 8);
  }

  static int write(ForeignGetters value, Uint8List buf) {
    final handle = lower(value);
    buf.buffer.asByteData(buf.offsetInBytes).setInt64(0, handle);
    return 8;
  }

  static int allocationSize(ForeignGetters value) {
    return 8; // Just a handle (int64).
  }
}

/// Converter for StoredForeignStringifier callback interface.
class FfiConverterCallbackInterfaceStoredForeignStringifier {
  static final _handleMap = UniffiHandleMap<StoredForeignStringifier>();

  static StoredForeignStringifier lift(int handle) {
    return _handleMap.get(handle);
  }

  static int lower(StoredForeignStringifier value) {
    return _handleMap.insert(value);
  }

  static LiftRetVal<StoredForeignStringifier> read(Uint8List buf) {
    final handle = buf.buffer.asByteData(buf.offsetInBytes).getInt64(0);
    return LiftRetVal(lift(handle), 8);
  }

  static int write(StoredForeignStringifier value, Uint8List buf) {
    final handle = lower(value);
    buf.buffer.asByteData(buf.offsetInBytes).setInt64(0, handle);
    return 8;
  }

  static int allocationSize(StoredForeignStringifier value) {
    return 8; // Just a handle (int64).
  }
}

/// We must define callback signatures and vtables in Dart that match what Rust expects.
///
/// The Rust code expects functions like these:
/// get_bool: (handle: Long, v: Byte, argumentTwo: Byte, outReturn: Byte*, callStatus: *RustCallStatus)
/// We'll define native callback types and a VTable struct in Dart.
/// NOTE: The exact signatures must match the Rust-generated headers.

typedef UniffiCallbackInterfaceForeignGettersMethod0 = Void Function(
    Uint64, Int8, Int8, Pointer<Int8>, Pointer<RustCallStatus>);
typedef UniffiCallbackInterfaceForeignGettersMethod0Dart = void Function(
    int, int, int, Pointer<Int8>, Pointer<RustCallStatus>);

typedef UniffiCallbackInterfaceForeignGettersMethod1 = Void Function(Uint64,
    RustBuffer, Int8, Pointer<RustBuffer>, Pointer<RustCallStatus>);
typedef UniffiCallbackInterfaceForeignGettersMethod1Dart = void Function(
    int, RustBuffer, int, Pointer<RustBuffer>, Pointer<RustCallStatus>);

typedef UniffiCallbackInterfaceForeignGettersMethod2 = Void Function(Uint64,
    RustBuffer, Bool, Pointer<RustBuffer>, Pointer<RustCallStatus>);
typedef UniffiCallbackInterfaceForeignGettersMethod2Dart = void Function(
    int, RustBuffer, bool, Pointer<RustBuffer>, Pointer<RustCallStatus>);


typedef UniffiCallbackInterfaceForeignGettersMethod4 = Void Function(
    Uint64, RustBuffer, Pointer<Void>, Pointer<RustCallStatus>);
typedef UniffiCallbackInterfaceForeignGettersMethod4Dart = void Function(
    int, RustBuffer, Pointer<Void>, Pointer<RustCallStatus>);
// Similarly define other methods...
// get_option, get_list, get_nothing
// For brevity, define all needed.

typedef UniffiCallbackInterfaceForeignGettersFree = Void Function(Uint64);
typedef UniffiCallbackInterfaceForeignGettersFreeDart = void Function(int);

final class UniffiVTableCallbackInterfaceForeignGetters extends Struct {
  external Pointer<NativeFunction<UniffiCallbackInterfaceForeignGettersMethod0>>
      getBool;
  external Pointer<NativeFunction<UniffiCallbackInterfaceForeignGettersMethod2>>
      getString;
  external Pointer<NativeFunction<UniffiCallbackInterfaceForeignGettersMethod2>>
      getOption; // same signature as getString but returns RustBuffer
  external Pointer<NativeFunction<UniffiCallbackInterfaceForeignGettersMethod2>>
      getList;
  external Pointer<NativeFunction<UniffiCallbackInterfaceForeignGettersMethod4>>
      getNothing;
  external Pointer<NativeFunction<UniffiCallbackInterfaceForeignGettersFree>>
      uniffiFree;
}

void foreignGettersGetBool(int uniffiHandle, int v, int argumentTwo,
    Pointer<Int8> outReturn, Pointer<RustCallStatus> callStatus) {
  final status = callStatus.ref;
  try {
    final obj = FfiConverterCallbackInterfaceForeignGetters._handleMap
        .get(uniffiHandle);
    final result = obj.getBool(v == 1, argumentTwo == 1);
    outReturn.value = result ? 1 : 0;
  } catch (e) {
    status.code = CALL_UNEXPECTED_ERROR;
    status.errorBuf = FfiConverterString.lower(e.toString());
  }
}

void foreignGettersGetString(
    int uniffiHandle,
    RustBuffer vBuffer,
    bool arg2,
    Pointer<RustBuffer> outReturn,
    Pointer<RustCallStatus> callStatus) {
  final status = callStatus.ref;
  try {
    final obj = FfiConverterCallbackInterfaceForeignGetters._handleMap
        .get(uniffiHandle);
    // Lift the arguments
    final v = FfiConverterString.lift(vBuffer);
    print(v);
    // Call the Dart method
    final result = obj.getString(v, arg2);
    // Lower the result into RustBuffer
    outReturn.ref = FfiConverterString.lower(result);
    status.code = CALL_SUCCESS;
    print("result: $result");
    print("outreturn: $outReturn");
  } catch (e) {
    status.code = CALL_UNEXPECTED_ERROR;
    status.errorBuf = FfiConverterString.lower(e.toString());
  }
}

void foreignGettersGetOption(
    int uniffiHandle,
    RustBuffer vBuffer,
    bool arg2,
    Pointer<RustBuffer> outReturn,
    Pointer<RustCallStatus> callStatus) {
  final status = callStatus.ref;
  try {
    final obj = FfiConverterCallbackInterfaceForeignGetters._handleMap
        .get(uniffiHandle);
    final v = FfiConverterOptionalString.lift(vBuffer);
    final argumentTwo = arg2;
    // Call the Dart method
    final result = obj.getOption(v, argumentTwo);
    // Lower the result into RustBuffer
    if (result == null) {
      outReturn.ref = toRustBuffer(Uint8List.fromList([0]));
    } else {
      final lowered = FfiConverterOptionalString.lower(result);
      outReturn.ref = toRustBuffer(lowered.asUint8List());
    }
  } catch (e) {
    status.code = CALL_UNEXPECTED_ERROR;
    status.errorBuf = FfiConverterString.lower(e.toString());
  }
}

void foreignGettersGetList(
    int uniffiHandle,
    RustBuffer vBuffer,
    bool arg2,
    Pointer<RustBuffer> outReturn,
    Pointer<RustCallStatus> callStatus) {
  print('foreignGettersGetList');
  final status = callStatus.ref;
  try {
    final obj = FfiConverterCallbackInterfaceForeignGetters._handleMap
        .get(uniffiHandle);
    // Lift the arguments
    final v = FfiConverterSequenceInt32.lift(vBuffer);
    final argumentTwo = arg2;
    // Call the Dart method
    final result = obj.getList(v, argumentTwo);
    // Lower the result into RustBuffer
    outReturn.ref = FfiConverterSequenceInt32.lower(result);
  } catch (e) {
    status.code = CALL_UNEXPECTED_ERROR;
    status.errorBuf = FfiConverterString.lower(e.toString());
  }
}

void foreignGettersGetNothing(int uniffiHandle, RustBuffer vBuffer,
    Pointer<Void> unused, Pointer<RustCallStatus> callStatus) {
  final status = callStatus.ref;
  try {
    final obj = FfiConverterCallbackInterfaceForeignGetters._handleMap
        .get(uniffiHandle);
    // Lift the argument
    final v = FfiConverterString.lift(vBuffer);
    // Call the Dart method
    obj.getNothing(v);
    // Indicate success
    status.code = CALL_SUCCESS;
  } catch (e) {
    status.code = CALL_UNEXPECTED_ERROR;
    status.errorBuf = FfiConverterString.lower(e.toString());
  }
}

void foreignGettersFreeCallback(int handle) {
  try {
    FfiConverterCallbackInterfaceForeignGetters._handleMap.remove(handle);
  } catch (e) {
    // Optionally log error, but do not return anything.
  }
}

final foreignGettersGetBoolPointer =
    Pointer.fromFunction<UniffiCallbackInterfaceForeignGettersMethod0>(
        foreignGettersGetBool);

final foreignGettersGetOptionPointer =
    Pointer.fromFunction<UniffiCallbackInterfaceForeignGettersMethod2>(
        foreignGettersGetOption);

final foreignGettersGetStringPointer =
    Pointer.fromFunction<UniffiCallbackInterfaceForeignGettersMethod2>(
        foreignGettersGetString);

final foreignGettersGetListPointer =
    Pointer.fromFunction<UniffiCallbackInterfaceForeignGettersMethod2>(
        foreignGettersGetList);

final foreignGettersGetNothingPointer =
    Pointer.fromFunction<UniffiCallbackInterfaceForeignGettersMethod4>(
        foreignGettersGetNothing);

// Each of these mirrors logic from Python/Kotlin/Swift examples:
// - Lift arguments
// - Call Dart method
// - Write return value or error

// For free callback, must match signature exactly: Void Function(Uint64)
final foreignGettersFreePointer =
    Pointer.fromFunction<UniffiCallbackInterfaceForeignGettersFree>(
        foreignGettersFreeCallback);

// Once implemented, create a static vtable instance and register it:
late final Pointer<UniffiVTableCallbackInterfaceForeignGetters> foreignGettersVTable;

void initForeignGettersVTable() {

  foreignGettersVTable =
      calloc<UniffiVTableCallbackInterfaceForeignGetters>();
  foreignGettersVTable.ref.getBool = foreignGettersGetBoolPointer;
  foreignGettersVTable.ref.getString = foreignGettersGetStringPointer;
  foreignGettersVTable.ref.getOption = foreignGettersGetOptionPointer;
  foreignGettersVTable.ref.getList = foreignGettersGetListPointer;
  foreignGettersVTable.ref.getNothing = foreignGettersGetNothingPointer;
  foreignGettersVTable.ref.uniffiFree = foreignGettersFreePointer;

  rustCall((status) {
    _UniffiLib.instance.uniffi_callbacks_fn_init_callback_vtable_foreigngetters(
      foreignGettersVTable, // Pass the pointer to the struct
    );
    checkCallStatus(NullRustCallStatusErrorHandler(), status);
  });
}

// Similarly for StoredForeignStringifier
typedef UniffiCallbackInterfaceStoredForeignStringifierMethod0 = Void Function(
    Uint64, Int32, Pointer<RustBuffer>, Pointer<RustCallStatus>);
typedef UniffiCallbackInterfaceStoredForeignStringifierMethod0Dart = void
    Function(int, int, Pointer<RustBuffer>, Pointer<RustCallStatus>);

typedef UniffiCallbackInterfaceStoredForeignStringifierMethod1 = Void Function(
    Uint64, Pointer<RustBuffer>, Pointer<RustBuffer>, Pointer<RustCallStatus>);
typedef UniffiCallbackInterfaceStoredForeignStringifierMethod1Dart = void
    Function(int, RustBuffer, Pointer<RustBuffer>, Pointer<RustCallStatus>);

typedef UniffiCallbackInterfaceFree = Void Function(Uint64);
typedef UniffiCallbackInterfaceFreeDart = void Function(int);

final class UniffiVTableCallbackInterfaceStoredForeignStringifier extends Struct {
  external Pointer<
          NativeFunction<
              UniffiCallbackInterfaceStoredForeignStringifierMethod0>>
      fromSimpleType;
  external Pointer<
          NativeFunction<
              UniffiCallbackInterfaceStoredForeignStringifierMethod1>>
      fromComplexType;
  external Pointer<NativeFunction<UniffiCallbackInterfaceFree>> uniffiFree;
}

// We must implement the callback methods that Rust will call to reach Dart's ForeignGetters.
// These methods must:
// 1. Lookup the object from handle.
// 2. Call the appropriate Dart method.
// 3. Write return value / errors as needed.

// Similarly do for StoredForeignStringifier:
// implement fromSimpleType and fromComplexType callbacks, create a vtable, and register it.

void storedForeignStringifierFromSimpleType(int uniffiHandle, int value,
    Pointer<RustBuffer> outReturn, Pointer<RustCallStatus> callStatus) {
  final status = callStatus.ref;
  try {
    final obj = FfiConverterCallbackInterfaceStoredForeignStringifier._handleMap
        .get(uniffiHandle);
    // Call the Dart method
    final result = obj.fromSimpleType(value);
    // Lower the result into RustBuffer
    outReturn.ref = FfiConverterString.lower(result);
    // Indicate success
    status.code = CALL_SUCCESS;
  } catch (e) {
    status.code = CALL_UNEXPECTED_ERROR;
    status.errorBuf = FfiConverterString.lower(e.toString());
  }
}

void storedForeignStringifierFromComplexType(
    int uniffiHandle,
    Pointer<RustBuffer> valuesBuffer,
    Pointer<RustBuffer> outReturn,
    Pointer<RustCallStatus> callStatus) {
  final status = callStatus.ref;
  try {
    final obj = FfiConverterCallbackInterfaceStoredForeignStringifier._handleMap
        .get(uniffiHandle);
    // Lift the arguments
    final values = FfiConverterDoubleNullable.lift(valuesBuffer.ref);
    // Call the Dart method
    final result = obj.fromComplexType(values);
    // Lower the result into RustBuffer
    outReturn.ref = FfiConverterString.lower(result);
    // Indicate success
    status.code = CALL_SUCCESS;
  } catch (e) {
    status.code = CALL_UNEXPECTED_ERROR;
    status.errorBuf = FfiConverterString.lower(e.toString());
  }
}

final storedForeignStringifierFromComplexTypePointer = Pointer.fromFunction<
        UniffiCallbackInterfaceStoredForeignStringifierMethod1>(
    storedForeignStringifierFromComplexType);

final storedForeignStringifierFromSimpleTypePointer = Pointer.fromFunction<
        UniffiCallbackInterfaceStoredForeignStringifierMethod0>(
    storedForeignStringifierFromSimpleType);

void storedForeignStringifierFreeCallback(int handle) {
  try {
    FfiConverterCallbackInterfaceStoredForeignStringifier._handleMap
        .remove(handle);
  } catch (e) {
    // Optionally log error, but do not return anything.
  }
}

final storedForeignStringifierFreePointer =
    Pointer.fromFunction<UniffiCallbackInterfaceFree>(
  storedForeignStringifierFreeCallback,
  // Provide a default value in case of error
);

late final Pointer<UniffiVTableCallbackInterfaceStoredForeignStringifier>
    storedForeignStringifierVTable;

void initStoredForeignStringifierVTable() {
  storedForeignStringifierVTable =
      calloc<UniffiVTableCallbackInterfaceStoredForeignStringifier>();

  storedForeignStringifierVTable.ref.fromSimpleType = storedForeignStringifierFromSimpleTypePointer;
  storedForeignStringifierVTable.ref.fromComplexType = storedForeignStringifierFromComplexTypePointer;
  storedForeignStringifierVTable.ref.uniffiFree = storedForeignStringifierFreePointer;

  rustCall((status) {
    _UniffiLib.instance
        .uniffi_callbacks_fn_init_callback_vtable_storedforeignstringifier(
      storedForeignStringifierVTable, // Pass the pointer to the struct
    );
    checkCallStatus(NullRustCallStatusErrorHandler(), status);
  });
}

void registerAllVTables() {
  initForeignGettersVTable();
  initStoredForeignStringifierVTable();
}

// End

class FfiConverterBool {
  static bool lift(int value) {
    return value == 1;
  }

  static int lower(bool value) {
    return value ? 1 : 0;
  }

  static LiftRetVal<bool> read(Uint8List buf) {
    return LiftRetVal(FfiConverterBool.lift(buf.first), 1);
  }

  static RustBuffer lowerIntoRustBuffer(bool value) {
    return toRustBuffer(Uint8List.fromList([FfiConverterBool.lower(value)]));
  }

  static int allocationSize([bool value = false]) {
    return 1;
  }

  static int write(bool value, Uint8List buf) {
    buf.setAll(0, [value ? 1 : 0]);
    return allocationSize();
  }
}

class FfiConverterInt32 {
  static int lift(int value) => value;

  static LiftRetVal<int> read(Uint8List buf) {
    return LiftRetVal(buf.buffer.asByteData(buf.offsetInBytes).getInt32(0), 4);
  }

  static int lower(int value) => value;

  static int allocationSize([int value = 0]) {
    return 4;
  }

  static int write(int value, Uint8List buf) {
    buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, value);
    return FfiConverterInt32.allocationSize();
  }
}

class FfiConverterSequenceInt32 {
  static List<int> lift(RustBuffer buf) {
    return FfiConverterSequenceInt32.read(buf.asUint8List()).value;
  }

  static LiftRetVal<List<int>> read(Uint8List buf) {
    List<int> res = [];
    final length = buf.buffer.asByteData(buf.offsetInBytes).getInt32(0);
    int offset = buf.offsetInBytes + 4;
    for (var i = 0; i < length; i++) {
      final ret = FfiConverterInt32.read(Uint8List.view(buf.buffer, offset));
      offset += ret.bytesRead;
      res.add(ret.value);
    }
    return LiftRetVal(res, offset - buf.offsetInBytes);
  }

  static int write(List<int> value, Uint8List buf) {
    buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, value.length);
    int offset = buf.offsetInBytes + 4;
    for (var i = 0; i < value.length; i++) {
      offset +=
          FfiConverterInt32.write(value[i], Uint8List.view(buf.buffer, offset));
    }
    return offset - buf.offsetInBytes;
  }

  static int allocationSize(List<int> value) {
    return value
            .map((l) => FfiConverterInt32.allocationSize(l))
            .reduce((a, b) => a + b) +
        4;
  }

  static RustBuffer lower(List<int> value) {
    final buf = Uint8List(allocationSize(value));
    write(value, buf);
    return toRustBuffer(buf);
  }
}

const int UNIFFI_RUST_FUTURE_POLL_READY = 0;
const int UNIFFI_RUST_FUTURE_POLL_MAYBE_READY = 1;

typedef UniffiRustFutureContinuationCallback = Void Function(Uint64, Int8);

Future<T> uniffiRustCallAsync<T, F>(
  int Function() rustFutureFunc,
  void Function(int,
          Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>, int)
      pollFunc,
  F Function(int, Pointer<RustCallStatus>) completeFunc,
  void Function(int) freeFunc,
  T Function(F) liftFunc, [
  UniffiRustCallStatusErrorHandler? errorHandler,
]) async {
  final rustFuture = rustFutureFunc();
  final completer = Completer<int>();

  late final NativeCallable<UniffiRustFutureContinuationCallback> callback;

  void poll() {
    pollFunc(
      rustFuture,
      callback.nativeFunction,
      0,
    );
  }

  void onResponse(int _idx, int pollResult) {
    if (pollResult == UNIFFI_RUST_FUTURE_POLL_READY) {
      completer.complete(pollResult);
    } else {
      poll();
    }
  }

  callback =
      NativeCallable<UniffiRustFutureContinuationCallback>.listener(onResponse);

  try {
    poll();
    await completer.future;
    callback.close();

    final status = calloc<RustCallStatus>();
    try {
      final result = completeFunc(rustFuture, status);

      return liftFunc(result);
    } finally {
      calloc.free(status);
    }
  } finally {
    freeFunc(rustFuture);
  }
}

class _UniffiLib {
  _UniffiLib._();

  static final DynamicLibrary _dylib = _open();

  static DynamicLibrary _open() {
    if (Platform.isAndroid) return DynamicLibrary.open("libcallbacks.so");
    if (Platform.isIOS) return DynamicLibrary.executable();
    if (Platform.isLinux) return DynamicLibrary.open("libcallbacks.so");
    if (Platform.isMacOS) return DynamicLibrary.open("libcallbacks.dylib");
    if (Platform.isWindows) return DynamicLibrary.open("callbacks.dll");
    throw UnsupportedError(
        "Unsupported platform: \${Platform.operatingSystem}");
  }

  static final _UniffiLib instance = _UniffiLib._();

  late final Pointer<Void> Function(Pointer<Void>, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_clone_rustgetters = _dylib.lookupFunction<
              Pointer<Void> Function(Pointer<Void>, Pointer<RustCallStatus>),
              Pointer<Void> Function(Pointer<Void>, Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_clone_rustgetters");
  late final void Function(Pointer<Void>, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_free_rustgetters = _dylib.lookupFunction<
          Void Function(Pointer<Void>, Pointer<RustCallStatus>),
          void Function(Pointer<Void>,
              Pointer<RustCallStatus>)>("uniffi_callbacks_fn_free_rustgetters");
  late final Pointer<Void> Function(Pointer<RustCallStatus>)
      uniffi_callbacks_fn_constructor_rustgetters_new = _dylib.lookupFunction<
              Pointer<Void> Function(Pointer<RustCallStatus>),
              Pointer<Void> Function(Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_constructor_rustgetters_new");
  late final int Function(Pointer<Void>, int, int, int, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_method_rustgetters_get_bool = _dylib.lookupFunction<
              Int8 Function(
                  Pointer<Void>, Uint64, Int8, Int8, Pointer<RustCallStatus>),
              int Function(
                  Pointer<Void>, int, int, int, Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_method_rustgetters_get_bool");
  late final RustBuffer Function(
          Pointer<Void>, int, RustBuffer, int, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_method_rustgetters_get_list = _dylib.lookupFunction<
              RustBuffer Function(Pointer<Void>, Uint64, RustBuffer, Int8,
                  Pointer<RustCallStatus>),
              RustBuffer Function(Pointer<Void>, int, RustBuffer, int,
                  Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_method_rustgetters_get_list");
  late final void Function(
          Pointer<Void>, int, RustBuffer, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_method_rustgetters_get_nothing =
      _dylib.lookupFunction<
              Void Function(
                  Pointer<Void>, Uint64, RustBuffer, Pointer<RustCallStatus>),
              void Function(
                  Pointer<Void>, int, RustBuffer, Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_method_rustgetters_get_nothing");
  late final RustBuffer Function(
          Pointer<Void>, int, RustBuffer, int, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_method_rustgetters_get_option = _dylib.lookupFunction<
              RustBuffer Function(Pointer<Void>, Uint64, RustBuffer, Int8,
                  Pointer<RustCallStatus>),
              RustBuffer Function(Pointer<Void>, int, RustBuffer, int,
                  Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_method_rustgetters_get_option");
  late final RustBuffer Function(
          Pointer<Void>, int, RustBuffer, int, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_method_rustgetters_get_string = _dylib.lookupFunction<
              RustBuffer Function(Pointer<Void>, Uint64, RustBuffer, Int8,
                  Pointer<RustCallStatus>),
              RustBuffer Function(Pointer<Void>, int, RustBuffer, int,
                  Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_method_rustgetters_get_string");
  late final RustBuffer Function(
          Pointer<Void>, RustBuffer, RustBuffer, int, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_method_rustgetters_get_string_optional_callback =
      _dylib.lookupFunction<
              RustBuffer Function(Pointer<Void>, RustBuffer, RustBuffer, Int8,
                  Pointer<RustCallStatus>),
              RustBuffer Function(Pointer<Void>, RustBuffer, RustBuffer, int,
                  Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_method_rustgetters_get_string_optional_callback");
  late final Pointer<Void> Function(Pointer<Void>, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_clone_ruststringifier = _dylib.lookupFunction<
              Pointer<Void> Function(Pointer<Void>, Pointer<RustCallStatus>),
              Pointer<Void> Function(Pointer<Void>, Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_clone_ruststringifier");
  late final void Function(Pointer<Void>, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_free_ruststringifier = _dylib.lookupFunction<
              Void Function(Pointer<Void>, Pointer<RustCallStatus>),
              void Function(Pointer<Void>, Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_free_ruststringifier");
  late final Pointer<Void> Function(int, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_constructor_ruststringifier_new =
      _dylib.lookupFunction<
              Pointer<Void> Function(Uint64, Pointer<RustCallStatus>),
              Pointer<Void> Function(int, Pointer<RustCallStatus>)>(
          "uniffi_callbacks_fn_constructor_ruststringifier_new");
  late final RustBuffer Function(Pointer<Void>, int, Pointer<RustCallStatus>)
      uniffi_callbacks_fn_method_ruststringifier_from_simple_type = _dylib
          .lookupFunction<
                  RustBuffer Function(
                      Pointer<Void>, Int32, Pointer<RustCallStatus>),
                  RustBuffer Function(
                      Pointer<Void>, int, Pointer<RustCallStatus>)>(
              "uniffi_callbacks_fn_method_ruststringifier_from_simple_type");
  late final void Function(
    Pointer<UniffiVTableCallbackInterfaceForeignGetters>,
  ) uniffi_callbacks_fn_init_callback_vtable_foreigngetters =
      _dylib.lookupFunction<
          Void Function(
            Pointer<UniffiVTableCallbackInterfaceForeignGetters>,
          ),
          void Function(
            Pointer<UniffiVTableCallbackInterfaceForeignGetters>,
          )>("uniffi_callbacks_fn_init_callback_vtable_foreigngetters");
  late final void Function(
    Pointer<UniffiVTableCallbackInterfaceStoredForeignStringifier>,
  ) uniffi_callbacks_fn_init_callback_vtable_storedforeignstringifier =
      _dylib.lookupFunction<
              Void Function(
                Pointer<UniffiVTableCallbackInterfaceStoredForeignStringifier>,
              ),
              void Function(
                Pointer<UniffiVTableCallbackInterfaceStoredForeignStringifier>,
              )>(
          "uniffi_callbacks_fn_init_callback_vtable_storedforeignstringifier");
  late final RustBuffer Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rustbuffer_alloc = _dylib.lookupFunction<
          RustBuffer Function(Uint64, Pointer<RustCallStatus>),
          RustBuffer Function(
              int, Pointer<RustCallStatus>)>("ffi_callbacks_rustbuffer_alloc");
  late final RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>)
      ffi_callbacks_rustbuffer_from_bytes = _dylib.lookupFunction<
          RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>),
          RustBuffer Function(ForeignBytes,
              Pointer<RustCallStatus>)>("ffi_callbacks_rustbuffer_from_bytes");
  late final void Function(RustBuffer, Pointer<RustCallStatus>)
      ffi_callbacks_rustbuffer_free = _dylib.lookupFunction<
          Void Function(RustBuffer, Pointer<RustCallStatus>),
          void Function(RustBuffer,
              Pointer<RustCallStatus>)>("ffi_callbacks_rustbuffer_free");
  late final RustBuffer Function(RustBuffer, int, Pointer<RustCallStatus>)
      ffi_callbacks_rustbuffer_reserve = _dylib.lookupFunction<
          RustBuffer Function(RustBuffer, Uint64, Pointer<RustCallStatus>),
          RustBuffer Function(RustBuffer, int,
              Pointer<RustCallStatus>)>("ffi_callbacks_rustbuffer_reserve");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_u8 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_u8");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_u8 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_u8");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_u8 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_u8");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_u8 = _dylib.lookupFunction<
              Uint8 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_u8");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_i8 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_i8");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_i8 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_i8");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_i8 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_i8");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_i8 = _dylib.lookupFunction<
              Int8 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_i8");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_u16 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_u16");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_u16 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_u16");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_u16 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_u16");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_u16 = _dylib.lookupFunction<
              Uint16 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_u16");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_i16 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_i16");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_i16 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_i16");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_i16 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_i16");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_i16 = _dylib.lookupFunction<
              Int16 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_i16");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_u32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_u32");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_u32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_u32");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_u32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_u32");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_u32 = _dylib.lookupFunction<
              Uint32 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_u32");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_i32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_i32");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_i32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_i32");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_i32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_i32");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_i32 = _dylib.lookupFunction<
              Int32 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_i32");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_u64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_u64");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_u64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_u64");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_u64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_u64");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_u64 = _dylib.lookupFunction<
              Uint64 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_u64");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_i64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_i64");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_i64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_i64");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_i64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_i64");
  late final int Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_i64 = _dylib.lookupFunction<
              Int64 Function(Uint64, Pointer<RustCallStatus>),
              int Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_i64");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_f32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_f32");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_f32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_f32");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_f32 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_f32");
  late final double Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_f32 = _dylib.lookupFunction<
              Float Function(Uint64, Pointer<RustCallStatus>),
              double Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_f32");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_f64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_f64");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_f64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_f64");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_f64 = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_f64");
  late final double Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_f64 = _dylib.lookupFunction<
              Double Function(Uint64, Pointer<RustCallStatus>),
              double Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_f64");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_pointer = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_pointer");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_pointer = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_pointer");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_pointer = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_pointer");
  late final Pointer<Void> Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_pointer = _dylib.lookupFunction<
              Pointer<Void> Function(Uint64, Pointer<RustCallStatus>),
              Pointer<Void> Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_pointer");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_rust_buffer = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_rust_buffer");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_rust_buffer = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_rust_buffer");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_rust_buffer = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_rust_buffer");
  late final RustBuffer Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_rust_buffer = _dylib.lookupFunction<
              RustBuffer Function(Uint64, Pointer<RustCallStatus>),
              RustBuffer Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_rust_buffer");
  late final void Function(
    int,
    Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
    int,
  ) ffi_callbacks_rust_future_poll_void = _dylib.lookupFunction<
      Void Function(
        Uint64,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        Uint64,
      ),
      void Function(
        int,
        Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>,
        int,
      )>("ffi_callbacks_rust_future_poll_void");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_cancel_void = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_cancel_void");
  late final void Function(
    int,
  ) ffi_callbacks_rust_future_free_void = _dylib.lookupFunction<
      Void Function(
        Uint64,
      ),
      void Function(
        int,
      )>("ffi_callbacks_rust_future_free_void");
  late final void Function(int, Pointer<RustCallStatus>)
      ffi_callbacks_rust_future_complete_void = _dylib.lookupFunction<
              Void Function(Uint64, Pointer<RustCallStatus>),
              void Function(int, Pointer<RustCallStatus>)>(
          "ffi_callbacks_rust_future_complete_void");
  late final int Function()
      uniffi_callbacks_checksum_method_rustgetters_get_bool =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_rustgetters_get_bool");
  late final int Function()
      uniffi_callbacks_checksum_method_rustgetters_get_list =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_rustgetters_get_list");
  late final int Function()
      uniffi_callbacks_checksum_method_rustgetters_get_nothing =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_rustgetters_get_nothing");
  late final int Function()
      uniffi_callbacks_checksum_method_rustgetters_get_option =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_rustgetters_get_option");
  late final int Function()
      uniffi_callbacks_checksum_method_rustgetters_get_string =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_rustgetters_get_string");
  late final int Function()
      uniffi_callbacks_checksum_method_rustgetters_get_string_optional_callback =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_rustgetters_get_string_optional_callback");
  late final int Function()
      uniffi_callbacks_checksum_method_ruststringifier_from_simple_type =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_ruststringifier_from_simple_type");
  late final int Function()
      uniffi_callbacks_checksum_constructor_rustgetters_new =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_constructor_rustgetters_new");
  late final int Function()
      uniffi_callbacks_checksum_constructor_ruststringifier_new =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_constructor_ruststringifier_new");
  late final int Function()
      uniffi_callbacks_checksum_method_foreigngetters_get_bool =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_foreigngetters_get_bool");
  late final int Function()
      uniffi_callbacks_checksum_method_foreigngetters_get_string =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_foreigngetters_get_string");
  late final int Function()
      uniffi_callbacks_checksum_method_foreigngetters_get_option =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_foreigngetters_get_option");
  late final int Function()
      uniffi_callbacks_checksum_method_foreigngetters_get_list =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_foreigngetters_get_list");
  late final int Function()
      uniffi_callbacks_checksum_method_foreigngetters_get_nothing =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_foreigngetters_get_nothing");
  late final int Function()
      uniffi_callbacks_checksum_method_storedforeignstringifier_from_simple_type =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_storedforeignstringifier_from_simple_type");
  late final int Function()
      uniffi_callbacks_checksum_method_storedforeignstringifier_from_complex_type =
      _dylib.lookupFunction<Uint16 Function(), int Function()>(
          "uniffi_callbacks_checksum_method_storedforeignstringifier_from_complex_type");
  late final int Function() ffi_callbacks_uniffi_contract_version =
      _dylib.lookupFunction<Uint32 Function(), int Function()>(
          "ffi_callbacks_uniffi_contract_version");

  static void _checkApiVersion() {
    final bindingsVersion = 26;
    final scaffoldingVersion =
        _UniffiLib.instance.ffi_callbacks_uniffi_contract_version();
    if (bindingsVersion != scaffoldingVersion) {
      throw UniffiInternalError.panicked(
          "UniFFI contract version mismatch: bindings version \$bindingsVersion, scaffolding version \$scaffoldingVersion");
    }
  }

  static void _checkApiChecksums() {
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_rustgetters_get_bool() !=
        35483) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_rustgetters_get_list() !=
        62422) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_rustgetters_get_nothing() !=
        21292) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_rustgetters_get_option() !=
        33488) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_rustgetters_get_string() !=
        21773) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_rustgetters_get_string_optional_callback() !=
        17930) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_ruststringifier_from_simple_type() !=
        12348) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_constructor_rustgetters_new() !=
        10154) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_constructor_ruststringifier_new() !=
        25564) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_foreigngetters_get_bool() !=
        45414) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_foreigngetters_get_string() !=
        27261) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_foreigngetters_get_option() !=
        11248) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_foreigngetters_get_list() !=
        31592) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_foreigngetters_get_nothing() !=
        62279) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_storedforeignstringifier_from_simple_type() !=
        25876) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
    if (_UniffiLib.instance
            .uniffi_callbacks_checksum_method_storedforeignstringifier_from_complex_type() !=
        17334) {
      throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
    }
  }
}

void initialize() {
  _UniffiLib._open();
}

void ensureInitialized() {
  _UniffiLib._checkApiVersion();
  _UniffiLib._checkApiChecksums();
}
