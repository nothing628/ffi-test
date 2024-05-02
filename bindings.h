/* Text to put at the beginning of the generated file. Probably a license. */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

typedef struct CAPI_ArrResult CAPI_ArrResult;

typedef struct CAPI_WatermarkTask CAPI_WatermarkTask;

 uint32_t add(uint32_t A) ;

 uint32_t add_array(const uint32_t *BytsPtr, size_t BytsLen) ;

 struct CAPI_WatermarkTask *create_watermarktask(void) ;

 void destroy_arr_result(struct CAPI_ArrResult *Ptr) ;

 void destroy_watermarktask(struct CAPI_WatermarkTask *Ptr) ;

 struct CAPI_ArrResult *get_section_jpeg(const uint8_t *BytsPtr, size_t BytsLen, uint32_t X, uint32_t Y, uint32_t W, uint32_t H) ;

 struct CAPI_ArrResult *get_section_webp(const uint8_t *BytsPtr, size_t BytsLen, uint32_t X, uint32_t Y, uint32_t W, uint32_t H) ;

 size_t len_arr_result(struct CAPI_ArrResult *Ptr) ;

 const uint8_t *read_arr_result(struct CAPI_ArrResult *Ptr, size_t Len) ;

 uint32_t *ret_arr(void) ;

 void set_position_watermark(struct CAPI_WatermarkTask *Ptr, uint32_t X, uint32_t Y, uint8_t OriginX, uint8_t OriginY) ;

 uint32_t set_target_jpeg(struct CAPI_WatermarkTask *Ptr, const uint8_t *BytsPtr, size_t BytsLen) ;

 uint32_t set_target_webp(struct CAPI_WatermarkTask *Ptr, const uint8_t *BytsPtr, size_t BytsLen) ;

 uint32_t set_watermark_jpeg(struct CAPI_WatermarkTask *Ptr, const uint8_t *BytsPtr, size_t BytsLen) ;

 uint32_t set_watermark_webp(struct CAPI_WatermarkTask *Ptr, const uint8_t *BytsPtr, size_t BytsLen) ;

/* Text to put at the end of the generated file */
