from google.protobuf.descriptor_pb2 import FileDescriptorSet
from google.protobuf import text_format
import subprocess

DESCRIPTOR_SET_FILE_PATH = 'descriptor_set.bin'
SRC_DIR = 'src/'
MESSAGE_PATH = SRC_DIR + 'test_message.proto'
DUMP_FILE_PATH = "descriptor_set.textproto"

protoc_ret = subprocess.run([
    'protoc',
    '--include_imports',
    '--include_source_info',
    '-o', DESCRIPTOR_SET_FILE_PATH,
    '-I', SRC_DIR,
    MESSAGE_PATH,
])
assert protoc_ret.returncode == 0

with open(DESCRIPTOR_SET_FILE_PATH, 'rb') as f:
    data = f.read()

fds = FileDescriptorSet()
fds.ParseFromString(data)

with open(DUMP_FILE_PATH, "w") as f:
    f.write(text_format.MessageToString(fds))
