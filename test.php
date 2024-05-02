<?php

include "./vendor/autoload.php";

use FFI\Scalar\Type;

$ffi = FFI::cdef(file_get_contents("./bindings.h"), "target/release/libffi_test.so");
// $result = $ffi->add(12);

// var_dump($result);

$inp_arr = Type::uint32Array([34, 35], false);
$inp_ptr = FFI::addr($inp_arr[0]);

$result = $ffi->add_array($inp_ptr, 2);

// var_dump($result, $inp_arr);
FFI::free($inp_ptr);
FFI::free($inp_arr);

// for ($i = 0; $i < 100000000;$i++) {
//     $result = $ffi->ret_arr();
//     FFI::free($result);
// }
// var_dump($result);

function cropWebp(\FFI $ffi)
{
    $fileContent = file_get_contents("./test.webp");
    $fileBytes = unpack("C*", $fileContent);
    $fileLength = count($fileBytes);

    $inp_arr = Type::uint8Array($fileBytes, false);
    $result_ptr = $ffi->get_section_webp($inp_arr, $fileLength, 625, 175, 200, 200);
    $out_len = $ffi->len_arr_result($result_ptr);
    $out_arr = $ffi->read_arr_result($result_ptr, $out_len);

    $tmp = [];

    for ($i = 0; $i < $out_len; $i++) {
        $tmp[] = $out_arr[$i];
    }

    $fileOut = implode(array_map("chr", $tmp));
    $ffi->destroy_arr_result($result_ptr);

    file_put_contents("crop.webp", $fileOut);
    FFI::free($inp_arr);
}

function cropJpeg(\FFI $ffi)
{
    $fileContent = file_get_contents("./test.jpeg");
    $fileBytes = unpack("C*", $fileContent);
    $fileLength = count($fileBytes);

    $inp_arr = Type::uint8Array($fileBytes, false);
    $result_ptr = $ffi->get_section_jpeg($inp_arr, $fileLength, 625, 175, 200, 200);
    $out_len = $ffi->len_arr_result($result_ptr);
    $out_arr = $ffi->read_arr_result($result_ptr, $out_len);

    $tmp = [];

    for ($i = 0; $i < $out_len; $i++) {
        $tmp[] = $out_arr[$i];
    }

    storeToFile($tmp, "crop.jpeg");
    $ffi->destroy_arr_result($result_ptr);

    FFI::free($inp_arr);
}

function storeToFile($tmp, $filename) {
    $fileOut = implode(array_map("chr", $tmp));
    file_put_contents($filename, $fileOut);
}

function testWatermark(\FFI $ffi)
{
    $targetContent = file_get_contents("./test.webp");
    $targetBytes = unpack("C*", $targetContent);
    $targetArr = Type::uint8Array($targetBytes, false);
    $targetLen = count($targetBytes);
    $watermarkContent = file_get_contents("./watermark.webp");
    $watermarkBytes = unpack("C*", $watermarkContent);
    $watermarkArr = Type::uint8Array($watermarkBytes, false);
    $watermarkLen = count($watermarkBytes);

    $watermarkTask = $ffi->create_watermarktask();
    $arrResult = $ffi->create_arr_result();
    $arrOld = $ffi->create_arr_result();

    $ffi->set_position_watermark($watermarkTask, 40, 40, 1, 1);
    $ffi->set_target_webp($watermarkTask, $targetArr, $targetLen);
    $ffi->set_watermark_webp($watermarkTask, $watermarkArr, $watermarkLen);
    $processResult = $ffi->process_watermark($watermarkTask);

    if ($processResult == 0) {
        $copyResult = $ffi->get_output_webp($watermarkTask, $arrResult);

        if ($copyResult == 0) {
            $resultLen = $ffi->len_arr_result($arrResult);
            $resultArr = $ffi->read_arr_result($arrResult, $resultLen);
            $tmp = [];

            for ($i = 0; $i < $resultLen; $i++) {
                $tmp[] = $resultArr[$i];
            }
            
            storeToFile($tmp, "crop.webp");
        }

        $copyResult = $ffi->get_old_section_webp($watermarkTask, $arrOld);

        if ($copyResult == 0) {
            $resultLen = $ffi->len_arr_result($arrOld);
            $resultArr = $ffi->read_arr_result($arrOld, $resultLen);
            $tmp = [];

            for ($i = 0; $i < $resultLen; $i++) {
                $tmp[] = $resultArr[$i];
            }
            
            storeToFile($tmp, "old.webp");
        }
    }

    $ffi->destroy_watermarktask($watermarkTask);
    $ffi->destroy_arr_result($arrResult);
    $ffi->destroy_arr_result($arrOld);
}

// cropWebp($ffi);
// cropJpeg($ffi);
testWatermark($ffi);
