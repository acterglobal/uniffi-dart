use genco::prelude::*;
use super::{Renderer, Renderable};

// TODO: Create struct that impliment Renderer/Renderable for primitives
// Implementations for primitive type helpers

pub fn generate_primitives_lifters() -> dart::Tokens {
    quote!{
        int liftInt8OrUint8(Uint8List buf, [int offset = 1]) {
            return buf.buffer.asByteData().getInt8(offset);
        }

        int liftInt16OrUint16(Uint8List buf, [int offset = 1]) {
            return buf.buffer.asByteData().getInt16(offset);
        }

        int liftInt32OrUint32(Uint8List buf, [int offset = 1]) {
            return buf.buffer.asByteData().getInt32(offset);
        }

        int liftInt64OrUint64(Uint8List buf, [int offset = 1]) {
            return buf.buffer.asByteData().getInt64(offset);
        }  

        double liftFloat32(Uint8List buf, [int offset = 1]) {
            return buf.buffer.asByteData().getFloat32(offset);
        }

        double liftFloat64(Uint8List buf, [int offset = 1]) {
            return buf.buffer.asByteData().getFloat64(offset);
        }

        bool liftBoolean(Uint8List buf, [int offset = 1]) {
            return buf.sublist(offset)[0] == 1;
        }
    }
}

// pub fn generate_primitives_lifters() -> dart::Tokens {
//     quote!{
//         int? liftInt8OrUint8(Uint8List buf, [int offset = 1]) {
//             return buf.isEmpty ? null : buf.buffer.asByteData().getInt8(offset);
//         }

//         int? liftInt16OrUint16(Uint8List buf, [int offset = 1]) {
//             return buf.isEmpty ? null : buf.buffer.asByteData().getInt16(offset);
//         }

//         int? liftInt32OrUint32(Uint8List buf, [int offset = 1]) {
//             return buf.isEmpty ? null : buf.buffer.asByteData().getInt32(offset);
//         }

//         int? liftInt64OrUint64(Uint8List buf, [int offset = 1]) {
//             return buf.isEmpty ? null : buf.buffer.asByteData().getInt64(offset);
//         }  

//         double? liftFloat32(Uint8List buf, [int offset = 1]) {
//             if (!buf.isEmpty) {
//                 double res = buf.buffer.asByteData().getFloat32(offset);
//                 res = double.parse(res.toStringAsFixed(6)); // Could adjust this later...
//                 return res;
//             } else {
//                 return null;
//             }
           
//            // return buf.isEmpty ? null : buf.buffer.asByteData().getFloat32(offset);
//         }
        
//         double? liftFloat64(Uint8List buf, [int offset = 1]) {
//             return buf.isEmpty ? null : buf.buffer.asByteData().getFloat64(offset);
//         }

//         bool? liftBoolean(Uint8List buf, [int offset = 1]) {
//             return buf.isEmpty ? null : (buf.sublist(offset)[0] == 1 ? true : false);
//         }
//     }
// }

pub fn generate_wrapper_lifters() -> dart::Tokens {
    quote! {
        class DataOffset<T> {
            final T? data;
            final int offset;
            DataOffset(this.data, this.offset);
        }
        
        // Todo!: Make this guy handle varaible strings
        DataOffset<T> liftVaraibleLength<T>(
            Uint8List buf, T? Function(Uint8List) lifter,
            [int offset = 1]) {
            final length = buf.buffer.asByteData().getInt32(offset); // the length in Uint8
            final liftedData = lifter(buf.sublist(offset + 4));
            return DataOffset(liftedData, length);
        }

        List<T> liftSequence<T>(Api api, Uint8List buf, Function(Uint8List, [int offset]) lifter, [int element_byte_size = 1,int offset = 0]) {
            List<T> res = [];
            buf = buf.sublist(offset);
            final length = buf.buffer.asByteData().getInt32(0);
            buf = buf.sublist(4);
            
            final element_byte_size = (buf.length ~/ length);
            offset = 0;
            
            for (var i = 0; i < length; i++) {
                offset = element_byte_size * i; // Update the offset for the next loop
                final item = lifter(buf, offset);
                res.add(item);
            }

            return res;
        }
    }
}

pub fn generate_primitives_lowerers() -> dart::Tokens {
    quote! {
        // TODO: Impliment lowerers for primitives        
        Uint8List createUint8ListFromInt(int value) {
            int length = value.bitLength ~/ 8 + 1;
        
            // Ensure the length is either 4 or 8
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

        Uint8List lowerUint8(Api api, int value) {
            final buf = Uint8List(1);
            final byteData = ByteData.sublistView(buf);
            byteData.setUint8(0, value);
            return Uint8List.fromList(buf.toList());
        }

        Uint8List lowerInt8(int value) {
            final buf = Uint8List(1);
            final byteData = ByteData.sublistView(buf);
            byteData.setInt8(0, value);
            return Uint8List.fromList(buf.toList());
        }

        Uint8List lowerUint16(int value) {
            final buf = Uint8List(2);
            final byteData = ByteData.sublistView(buf);
            byteData.setUint16(0, value);
            return Uint8List.fromList(buf.toList());
        }

        Uint8List lowerInt16(int value) {
            final buf = Uint8List(2);
            final byteData = ByteData.sublistView(buf);
            byteData.setInt16(0, value);
            return Uint8List.fromList(buf.toList());
        }

        Uint8List lowerFloat32(double value) {
            final buf = Uint8List(4);
            final byteData = ByteData.sublistView(buf);
            byteData.setFloat32(0, value, Endian.little);
            return Uint8List.fromList(buf.reversed.toList());
        }

        Uint8List lowerFloat64(double value) {
            final buf = Uint8List(8);
            final byteData = ByteData.sublistView(buf);
            byteData.setFloat64(0, value, Endian.little);
            return Uint8List.fromList(buf.reversed.toList());
        }
    }
}

pub fn generate_wrapper_lowerers() -> dart::Tokens {
    quote! {
        Uint8List lowerVaraibleLength<T>(Api api, T input, Uint8List Function(Api, T) lowerer) {
            final lowered = lowerer(api, input);
            final length = createUint8ListFromInt(lowered.length);
            Uint8List res = Uint8List(lowered.length + length.length);
            res.setAll(0, length);
            res.setAll(length.length, lowered);
            return res;
        }

        Uint8List lowerSequence<T, V>(Api api, List<T> input, Uint8List Function(Api, V) lowerer, int element_byte_size) {
          int capacity = input.length * element_byte_size;
          Uint8List items = Uint8List(capacity + 4); // Four bytes for the length
          int offset = 0;
        
          // Set the length of the vec
          items.setAll(offset, createUint8ListFromInt(capacity));
          offset += 4;
        
          for (var i = 0; i < input.length; i++) {
            items.setRange(
                offset, offset + element_byte_size, lowerer(api, input[i] as V));
            offset += element_byte_size;
          }
        
          print("Items from sequence");
          print(items);
          return items;
        }
    }
}
