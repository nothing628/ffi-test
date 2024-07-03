<script setup lang="ts">
import { onMounted, ref } from "vue";
import { add, get_replacement_jpeg, get_replacement_webp } from "drmwasm";

const image = ref();
const replace_img = ref();
const file_meta = ref("");
const watermark_x = ref(-1);
const watermark_y = ref(-1);
const watermark_w = ref(-1);
const watermark_h = ref(-1);
const replacement_img = ref<Blob | null>(null);

const readFile = async (file: File) => {
  const promise = new Promise<ArrayBuffer>((resolve, reject) => {
    const file_reader = new FileReader();
    file_reader.onload = () => {
      const file_content = file_reader.result;

      if (file_content) {
        resolve(file_content as ArrayBuffer);
      } else {
        reject("Cannot read file content");
      }
    };
    file_reader.readAsArrayBuffer(file);
  });

  return promise;
};

const drawImage = (file_content: Uint8Array, metadata: string) => {
  if (image.value) {
    const image_elem = image.value as HTMLImageElement;
    const file_blob = new Blob([file_content], { type: metadata });
    const file_url = URL.createObjectURL(file_blob);

    image_elem.src = file_url;
  }
};

const drawReplaceImg = (file_content: Uint8Array, metadata: string) => {
  if (replace_img.value) {
    const image_elem = replace_img.value as HTMLImageElement;
    const file_blob = new Blob([file_content], {
      type: metadata,
    });
    const file_url = URL.createObjectURL(file_blob);

    image_elem.src = file_url;
  }
};

const storeReplacementImage = (subimage_replacement: any, metadata: string) => {
  const subimage = subimage_replacement.real_img;

  watermark_h.value = subimage_replacement.width;
  watermark_w.value = subimage_replacement.height;
  watermark_x.value = subimage_replacement.x;
  watermark_y.value = subimage_replacement.y;
  // replacement_img.value = subimage_blob;
  drawReplaceImg(subimage, metadata);
};

const clearReplacementImage = () => {
  watermark_h.value = -1;
  watermark_w.value = -1;
  watermark_x.value = -1;
  watermark_y.value = -1;
  replacement_img.value = null;
};

const handleFile = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const files = target.files;

  if (files && files.length > 0) {
    const first_file = files.item(0);

    if (first_file) {
      const metadata = first_file.type;
      const file_content = await readFile(first_file);
      const file_uint8 = new Uint8Array(file_content);

      file_meta.value = metadata;

      try {
        if (metadata == "image/jpeg") {
          const subimage_replacement = get_replacement_jpeg(file_uint8);
          storeReplacementImage(subimage_replacement, metadata);
        } else {
          const subimage_replacement = get_replacement_webp(file_uint8);
          storeReplacementImage(subimage_replacement, metadata);
        }
      } catch (err) {
        console.log(err);
        clearReplacementImage();
      }

      drawImage(file_uint8, metadata);
    }
  }
};

const handleError = (err: any) => {
  console.log(err);
};

onMounted(() => {
  console.log(add(20));
});
</script>

<template>
  <div>
    <div>
      <input type="file" accept="image/jpeg,image/webp" @change="handleFile" />
      <img class="preview" ref="image" />
      <img class="replace" ref="replace_img" @error="handleError" />
    </div>
  </div>
</template>

<style scoped>
.preview {
  width: auto;
  height: 800px;
}
.replace {
  width: 200px;
  height: auto;
}
</style>
