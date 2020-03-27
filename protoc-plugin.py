#!/usr/bin/env python3

import sys
from google.protobuf.compiler import plugin_pb2 as plugin
from google.protobuf import text_format


def generate_code(request, response):
    options = request.proto_file[-1].message_type[0].field[-1].options
    exts = len(options.Extensions)
    sys.stderr.write("Number of extensions: {}\n".format(exts))
    sys.stderr.write("options: {}\n".format(options))
    with open('plugin_descriptor_set.textproto', 'w') as f:
        for proto_file in request.proto_file:
            f.write(text_format.MessageToString(proto_file))


if __name__ == '__main__':
    request = plugin.CodeGeneratorRequest()
    data = sys.stdin.buffer.read()
    request.ParseFromString(data)

    response = plugin.CodeGeneratorResponse()
    generate_code(request, response)

    output = response.SerializeToString()
    sys.stdout.buffer.write(output)
