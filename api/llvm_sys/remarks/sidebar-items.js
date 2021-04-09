initSidebarItems({"constant":[["REMARKS_API_VERSION",""]],"enum":[["LLVMRemarkOpaqueArg",""],["LLVMRemarkOpaqueDebugLoc",""],["LLVMRemarkOpaqueEntry",""],["LLVMRemarkOpaqueParser",""],["LLVMRemarkOpaqueString",""],["LLVMRemarkType",""]],"fn":[["LLVMRemarkArgGetDebugLoc","Returns the debug location that is attached to the value of this argument."],["LLVMRemarkArgGetKey","Returns the key of an argument. The key defines what the value is, and the same key can appear multiple times in the list of arguments."],["LLVMRemarkArgGetValue","Returns the value of an argument. This is a string that can contain newlines."],["LLVMRemarkDebugLocGetSourceColumn","Return the column in the source file for a debug location."],["LLVMRemarkDebugLocGetSourceFilePath","Return the path to the source file for a debug location."],["LLVMRemarkDebugLocGetSourceLine","Return the line in the source file for a debug location."],["LLVMRemarkEntryDispose","Free the resources used by the remark entry."],["LLVMRemarkEntryGetDebugLoc","Returns the debug location that is attached to this remark."],["LLVMRemarkEntryGetFirstArg","Get a new iterator to iterate over a remark's argument."],["LLVMRemarkEntryGetFunctionName","Get the name of the function being processed when the remark was emitted."],["LLVMRemarkEntryGetHotness","Return the hotness of the remark."],["LLVMRemarkEntryGetNextArg","Get the next argument in Remark from the position of It."],["LLVMRemarkEntryGetNumArgs","The number of arguments the remark holds."],["LLVMRemarkEntryGetPassName","Get the name of the pass that emitted this remark."],["LLVMRemarkEntryGetRemarkName","Get an identifier of the remark."],["LLVMRemarkEntryGetType","The type of the remark. For example, it can allow users to only keep the missed optimizations from the compiler."],["LLVMRemarkParserCreateBitstream",""],["LLVMRemarkParserCreateYAML","Creates a remark parser that can be used to parse the buffer located in Buf of size Size bytes."],["LLVMRemarkParserDispose",""],["LLVMRemarkParserGetErrorMessage","Returns a null-terminated string containing an error message."],["LLVMRemarkParserGetNext","Returns the next remark in the file."],["LLVMRemarkParserHasError","Returns `1` if the parser encountered an error while parsing the buffer."],["LLVMRemarkStringGetData","Returns the buffer holding the string."],["LLVMRemarkStringGetLen","Returns the size of the string."],["LLVMRemarkVersion","Returns the version of the remarks library."]],"type":[["LLVMRemarkArgRef","Element of the \"Args\" list. The key might give more information about what the semantics of the value are, e.g. \"Callee\" will tell you that the value is a symbol that names a function."],["LLVMRemarkDebugLocRef","DebugLoc containing File, Line and Column."],["LLVMRemarkEntryRef","A remark emitted by the compiler."],["LLVMRemarkParserRef",""],["LLVMRemarkStringRef","String containing a buffer and a length. The buffer is not guaranteed to be zero-terminated."]]});