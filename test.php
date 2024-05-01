<?php

$ffi = FFI::cdef(file_get_contents("./bindings.h"), "target/release/libffi_test.so");
$result = $ffi->add(12);

var_dump($result);

$inp_arr = FFI::new("int [2]", false);
$inp_arr[0] = 34;
$inp_arr[1] = 35;
$inp_ptr = FFI::addr($inp_arr[0]);

$result = $ffi->add_array($inp_ptr,2);

var_dump($result);
FFI::free($inp_ptr);
FFI::free($inp_arr);

for ($i = 0; $i < 100000000;$i++) {
    $result = $ffi->ret_arr();
    FFI::free($result);
}
var_dump($result);
