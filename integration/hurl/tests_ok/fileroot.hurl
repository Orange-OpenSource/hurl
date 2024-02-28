# --file-root access is currently used for input and output files that are referenced in a Hurl file.
# User explicitly sets a "file root" through the command line, which acts as the parent of relatives path used
# in the Hurl file. Without specifying this "file root", the Hurl file's directory is used as "file root".

GET http://localhost:8000/fileroot
[Options]
output: fileroot.bin
HTTP 200


POST http://localhost:8000/fileroot
file,fileroot.bin;
HTTP 200


# Paths are OK if they are descendant of the file root.
POST http://localhost:8000/fileroot
file,../build/fileroot.bin;
HTTP 200
