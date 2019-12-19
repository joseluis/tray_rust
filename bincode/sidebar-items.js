initSidebarItems({"fn":[["deserialize","Deserializes a slice of bytes into an object."],["deserialize_from","Deserializes an object directly from a `Buffer`ed Reader."],["serialize","Serializes a serializable object into a `Vec` of bytes."],["serialize_into","Serializes an object directly into a `Writer`."]],"mod":[["internal","A collection of serialization and deserialization functions that use the `serde` crate for the serializable and deserializable implementation."],["read_types","The types that the deserializer uses for optimizations"]],"struct":[["Bounded","A SizeLimit that restricts serialized or deserialized messages from exceeding a certain byte length."],["Infinite","A SizeLimit without a limit! Use this if you don't care about the size of encoded or decoded messages."]],"trait":[["SizeLimit","A limit on the amount of bytes that can be read or written."]],"type":[["Deserializer","A Deserializer that uses LittleEndian byteorder"],["Serializer","A Serializer that uses LittleEndian byteorder"]]});