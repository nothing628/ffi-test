<?php

include "./vendor/autoload.php";

use FFI\Scalar\Type;

$ffi = FFI::cdef(file_get_contents("./bindings.h"), "target/release/libffi_test.so");
// $result = $ffi->add(12);

// var_dump($result);

$inp_arr = Type::uint32Array([34, 35], false);
$inp_ptr = FFI::addr($inp_arr[0]);

$result = $ffi->add_array($inp_ptr, 2);

var_dump($result, $inp_arr);
FFI::free($inp_ptr);
FFI::free($inp_arr);

// for ($i = 0; $i < 100000000;$i++) {
//     $result = $ffi->ret_arr();
//     FFI::free($result);
// }
// var_dump($result);

$fileContent = file_get_contents("./test.webp");
$fileBytes = unpack("C*", $fileContent);
$fileLength = count($fileBytes);

var_dump($fileLength);

$inp_arr = Type::uint8Array($fileBytes, false);
$result_ptr = $ffi->get_section_webp($inp_arr, $fileLength, 625, 175, 200, 200);
$out_len = $ffi->len_arr_result($result_ptr);
$out_arr = $ffi->read_arr_result($result_ptr, $out_len);

$tmp = [];

for ($i = 0; $i < $out_len; $i++) {
    $tmp[] = $out_arr[$i];
}

$fileOut = implode(array_map("chr", $tmp));

file_put_contents("crop.webp", $fileOut);

$ffi->destroy_arr_result($result_ptr);
FFI::free($inp_arr);
