function urldecode() { : "${*//+/ }"; echo -e "${_//%/\\x}"; }
timeout --signal=KILL ${TIMEOUT} gcc -o main src/main.cpp -lstdc++ && ./main $(urldecode "${ARGS}")